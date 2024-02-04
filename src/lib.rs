use regex::Regex;
use serde::Serialize;

#[derive(Serialize)]
pub struct Task {
    pub name: String,
    pub completed: bool,
}

/// Parses the input text into a vector of `Task` objects.
pub fn parse_input(input: &str) -> Vec<Task> {
    let task_regex = Regex::new(r"^\s*-\s*\[(\s|x)]\s*(.*)").unwrap();

    input.lines().filter_map(|line| {
        task_regex.captures(line).map(|caps| {
            let completed = caps.get(1).map_or(false, |m| m.as_str() == "x");
            let name = caps.get(2).map_or("", |m| m.as_str()).to_string();
            Task { name, completed }
        })
    }).collect()
}
