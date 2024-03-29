use chrono::{Local, NaiveDate};
use regex::Regex;
use serde::{Deserialize, Serialize};
pub mod dates;
mod serializers {
    use chrono::NaiveDate;
    use serde::{Deserializer, Serializer};
    use std::fmt;

    // Implement the serialization function for NaiveDate
    pub fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => serializer.serialize_some(&d.format("%Y-%m-%d").to_string()),
            None => serializer.serialize_none(),
        }
    }

    // Implement the deserialization function for NaiveDate
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use chrono::format::ParseError;
        use serde::de::{self, Visitor};

        struct DateVisitor;

        impl<'de> Visitor<'de> for DateVisitor {
            type Value = Option<NaiveDate>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a formatted date string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                NaiveDate::parse_from_str(value, "%Y-%m-%d")
                    .map(Some)
                    .map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_option(DateVisitor)
    }
}

pub trait Filter {
    fn apply<'a>(&self, tasks: Vec<&'a Task>) -> Vec<&'a Task>;
}

pub struct OverdueFilter {
    pub show_overdue: bool,
}

impl Filter for OverdueFilter {
    fn apply<'a>(&self, tasks: Vec<&'a Task>) -> Vec<&'a Task> {
        if self.show_overdue {
            tasks // Directly return the tasks if filtering is not needed
        } else {
            tasks.into_iter().filter(|&task| !task.overdue).collect()
        }
    }
}

pub struct DateRangeFilter {
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}

impl Filter for DateRangeFilter {
    fn apply<'a>(&self, tasks: Vec<&'a Task>) -> Vec<&'a Task> {
        tasks.into_iter().filter(|&task| {
            // Match against the combination of 'from' date, 'to' date, and the task's due date
            match (&self.from_date, &self.to_date, &task.due) {
                (Some(from), Some(to), Some(date)) => date >= from && date <= to,
                (Some(from), None, Some(date)) => date >= from,
                (None, Some(to), Some(date)) => date <= to,
                (None, None, _) => true, // Include tasks when no date filter is applied
                (_, _, None) => false, // Exclude tasks without due dates when any date filter is applied
            }
        }).collect()
    }
}



pub struct FilterPipeline {
    pub filters: Vec<Box<dyn Filter>>,
}

impl FilterPipeline {
    pub fn new() -> Self {
        FilterPipeline {
            filters: Vec::new(),
        }
    }

    pub fn add_filter(&mut self, filter: Box<dyn Filter>) {
        self.filters.push(filter);
    }

    pub fn apply<'a>(&self, tasks: Vec<&'a Task>) -> Vec<&'a Task> {
        self.filters
            .iter()
            .fold(tasks, |acc, filter| filter.apply(acc))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    Highest,
    High,
    Medium,
    Low,
    Lowest,
    None, // Represents no specific priority
}

#[derive(Serialize)]
pub struct Task {
    pub name: String,
    pub completed: bool,
    #[serde(
        serialize_with = "serializers::serialize",
        deserialize_with = "serializers::deserialize",
        skip_serializing_if = "Option::is_none"
    )]
    pub due: Option<NaiveDate>,
    #[serde(
        serialize_with = "serializers::serialize",
        deserialize_with = "serializers::deserialize",
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled: Option<NaiveDate>,
    #[serde(
        serialize_with = "serializers::serialize",
        deserialize_with = "serializers::deserialize",
        skip_serializing_if = "Option::is_none"
    )]
    pub start: Option<NaiveDate>,
    pub overdue: bool,
    pub priority: Priority,
}

fn clean_description(description: &str) -> String {
    let re = Regex::new(r"\s+").unwrap(); // Matches one or more whitespace characters
    re.replace_all(description.trim(), " ").to_string()
}

pub fn parse_priority(description: &str) -> (String, Priority) {
    let (priority, signifier) = if description.contains("🔺") {
        (Priority::Highest, "🔺")
    } else if description.contains("⏫") {
        (Priority::High, "⏫")
    } else if description.contains("🔼") {
        (Priority::Medium, "🔼")
    } else if description.contains("🔽") {
        (Priority::Low, "🔽")
    } else if description.contains("⏬") {
        (Priority::Lowest, "⏬")
    } else {
        (Priority::None, "")
    };

    // Remove the signifier from the description to clean it up
    let clean_description = if !signifier.is_empty() {
        description.replace(signifier, "").trim().to_string()
    } else {
        description.to_string()
    };

    (clean_description, priority)
}

/// Parses the input text into a vector of `Task` objects.
pub fn parse_input(input: &str) -> Vec<Task> {
    let task_regex = Regex::new(r"^\s*-\s*\[(\s|x)]\s*(.*)").unwrap();
    let due_date_regex = Regex::new(r"📅 (\d{4}-\d{2}-\d{2})").unwrap();
    let scheduled_date_regex = Regex::new(r"⏳ (\d{4}-\d{2}-\d{2})").unwrap();
    let start_date_regex = Regex::new(r"🛫 (\d{4}-\d{2}-\d{2})").unwrap(); // Regex for start dates

    input
        .lines()
        .filter_map(|line| {
            task_regex.captures(line).map(|caps| {
                let completed = caps.get(1).map_or(false, |m| m.as_str() == "x");
                let mut name_with_potential_dates =
                    caps.get(2).map_or("", |m| m.as_str()).to_string();

                // Extract and parse the due date
                let due = parse_date(&due_date_regex, &name_with_potential_dates);
                // Extract and parse the scheduled date
                let scheduled = parse_date(&scheduled_date_regex, &name_with_potential_dates);
                // Extract and parse the start date
                let start = parse_date(&start_date_regex, &name_with_potential_dates);

                // Clean the task name by removing date strings
                name_with_potential_dates = remove_date_strings(
                    &[&due_date_regex, &scheduled_date_regex, &start_date_regex],
                    name_with_potential_dates,
                );

                let overdue = due.map_or(false, |due_date| due_date < Local::today().naive_local());

                let (description_without_priorities, priority) =
                    parse_priority(&name_with_potential_dates);

                // Clean up the remaining description
                let cleaned_description = clean_description(&description_without_priorities);

                Task {
                    name: cleaned_description,
                    completed,
                    due,
                    scheduled,
                    start,
                    overdue,
                    priority: priority,
                }
            })
        })
        .collect()
}

fn parse_date(date_regex: &Regex, text: &str) -> Option<NaiveDate> {
    date_regex.captures(text).and_then(|caps| {
        caps.get(1)
            .and_then(|m| NaiveDate::parse_from_str(m.as_str(), "%Y-%m-%d").ok())
    })
}

fn remove_date_strings(regexes: &[&Regex], mut text: String) -> String {
    for regex in regexes {
        text = regex.replace_all(&text, "").to_string();
    }
    text
}
