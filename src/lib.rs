
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
}

/// Parses the input text into a vector of `Task` objects.
pub fn parse_input(input: &str) -> Vec<Task> {
    let task_regex = Regex::new(r"^\s*-\s*\[(\s|x)]\s*(.*)").unwrap();
    let date_regex = Regex::new(r"📅 (\d{4}-\d{2}-\d{2})").unwrap();

    input.lines().filter_map(|line| {
        task_regex.captures(line).map(|caps| {
            let completed = caps.get(1).map_or(false, |m| m.as_str() == "x");
            let name_with_potential_date = caps.get(2).map_or("", |m| m.as_str());
            let name = date_regex.replace_all(name_with_potential_date, "").trim().to_string();

            // Attempt to extract and parse the due date
            let due = date_regex.captures(&name_with_potential_date).and_then(|dcaps| {
                dcaps.get(0).map(|m| {
                    let date_str = m.as_str().trim_start_matches('📅').trim();
                    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()
                }).flatten() // Correctly flattens to Option<NaiveDate>
            });

            Task { name, completed, due }
        })
    }).collect()
}

