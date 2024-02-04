use clap::{App, Arg};
use std::io::{self, Read};
use marktask::{FilterPipeline, OverdueFilter, parse_input, Task};
use serde_json;

fn main() {
    let matches = App::new("marktask")
        .version("0.1.0")
        .about("Processes Markdown tasks")
        .arg(Arg::with_name("json")
            .long("json")
            .help("Outputs the tasks in JSON format")
            .takes_value(false))
        .arg(Arg::with_name("overdue")
            .long("overdue")
            .value_name("BOOLEAN")
            .help("Filters tasks based on their overdue status. Defaults to true, showing all tasks.")
            .takes_value(true)
            .possible_values(&["true", "false"]))
        .get_matches();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");

    let tasks = parse_input(&input);
    let task_refs: Vec<&Task> = tasks.iter().collect();

    // Initialize the filter pipeline
    let mut pipeline = FilterPipeline::new();

    // Add filters based on command-line arguments
    if let Some(overdue_val) = matches.value_of("overdue") {
        let show_overdue = overdue_val != "false"; // Convert argument to boolean
        pipeline.add_filter(Box::new(OverdueFilter { show_overdue }));
    }

    // Apply the pipeline filters
    let filtered_tasks = pipeline.apply(task_refs);

    // Output logic based on the presence of the `--json` flag
    if matches.is_present("json") {
        println!("{}", serde_json::to_string(&filtered_tasks).expect("Failed to serialize tasks"));
    } else {
        for task in filtered_tasks {
            println!("{} - {}", if task.completed { "[x]" } else { "[ ]" }, task.name);
        }
    }
}
