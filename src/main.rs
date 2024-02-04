// main.rs
use clap::{App, Arg};
use std::io::{self, Read};
use marktask::parse_input;
use serde_json;

fn main() {
    let matches = App::new("mdtasks")
        .version("1.0")
        .about("Processes Markdown tasks")
        .arg(Arg::with_name("json")
            .long("json")
            .help("Outputs the tasks in JSON format"))
        .get_matches();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");

    let tasks = parse_input(&input);

    if matches.is_present("json") {
        // If the --json flag is present, output tasks as JSON
        println!("{}", serde_json::to_string(&tasks).expect("Failed to serialize tasks"));
    } else {
        // Otherwise, print tasks in the standard format
        for task in tasks {
            println!("{} - {}", if task.completed { "[x]" } else { "[ ]" }, task.name);
        }
    }
}
