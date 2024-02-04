
use regex::Regex;
use serde::{Serialize};
use chrono::NaiveDate;

mod serializers {
    use chrono::NaiveDate;
    use serde::{Deserializer, Serializer};
    use serde::ser::SerializeStruct;
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
        use serde::de::{self, Visitor};
        use chrono::format::ParseError;

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


#[derive(Serialize)]
pub struct Task {
    pub name: String,
    pub completed: bool,
    #[serde(serialize_with = "serializers::serialize", deserialize_with = "serializers::deserialize", skip_serializing_if = "Option::is_none")]
    pub due: Option<NaiveDate>,
    #[serde(serialize_with = "serializers::serialize", deserialize_with = "serializers::deserialize", skip_serializing_if = "Option::is_none")]
    pub scheduled: Option<NaiveDate>,
    #[serde(serialize_with = "serializers::serialize", deserialize_with = "serializers::deserialize", skip_serializing_if = "Option::is_none")]
    pub start: Option<NaiveDate>,

}

/// Parses the input text into a vector of `Task` objects.
pub fn parse_input(input: &str) -> Vec<Task> {
    let task_regex = Regex::new(r"^\s*-\s*\[(\s|x)]\s*(.*)").unwrap();
    let due_date_regex = Regex::new(r"📅 (\d{4}-\d{2}-\d{2})").unwrap();
    let scheduled_date_regex = Regex::new(r"⏳ (\d{4}-\d{2}-\d{2})").unwrap();
    let start_date_regex = Regex::new(r"🛫 (\d{4}-\d{2}-\d{2})").unwrap(); // Regex for start dates

    input.lines().filter_map(|line| {
        task_regex.captures(line).map(|caps| {
            let completed = caps.get(1).map_or(false, |m| m.as_str() == "x");
            let mut name_with_potential_dates = caps.get(2).map_or("", |m| m.as_str()).to_string();

            // Extract and parse the due date
            let due = parse_date(&due_date_regex, &name_with_potential_dates);
            // Extract and parse the scheduled date
            let scheduled = parse_date(&scheduled_date_regex, &name_with_potential_dates);
            // Extract and parse the start date
            let start = parse_date(&start_date_regex, &name_with_potential_dates);

            // Clean the task name by removing date strings
            name_with_potential_dates = remove_date_strings(&[&due_date_regex, &scheduled_date_regex, &start_date_regex], name_with_potential_dates);

            Task { name: name_with_potential_dates.trim().to_string(), completed, due, scheduled, start }
        })
    }).collect()
}

fn parse_date(date_regex: &Regex, text: &str) -> Option<NaiveDate> {
    date_regex.captures(text).and_then(|caps| {
        caps.get(1).and_then(|m| NaiveDate::parse_from_str(m.as_str(), "%Y-%m-%d").ok())
    })
}

fn remove_date_strings(regexes: &[&Regex], mut text: String) -> String {
    for regex in regexes {
        text = regex.replace_all(&text, "").to_string();
    }
    text
}