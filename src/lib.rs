// lib.rs

/// Represents a single task with a name and completion status.
use serde::Serialize;

#[derive(Serialize)]
pub struct Task {
    pub name: String,
    pub completed: bool,
}

/// Parses the input text into a vector of `Task` objects.
pub fn parse_input(input: &str) -> Vec<Task> {
    input.lines().map(|line| {
        // Determine if the task is completed based on the presence of "[x]" and extract the task name.
        let completed = line.contains("[x]");
        let name = line.trim_start_matches("- [ ] ")
            .trim_start_matches("- [x] ")
            .to_string();

        Task { name, completed }
    }).collect()
}
