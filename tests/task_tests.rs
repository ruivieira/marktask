use std::fs;
use std::path::PathBuf;
use marktask::parse_input;
use marktask::Task;
use chrono::NaiveDate;

#[test]
fn test_task_completion() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/data/tasks.txt");

    let input = fs::read_to_string(path)
        .expect("Failed to read tasks.txt");

    let tasks = parse_input(&input);

    let completed_tasks: Vec<&Task> = tasks.iter().filter(|t| t.completed).collect();
    let incomplete_tasks: Vec<&Task> = tasks.iter().filter(|t| !t.completed).collect();

    assert_eq!(completed_tasks.len(), 1, "There should be 2 completed tasks");
    assert_eq!(incomplete_tasks.len(), 3, "There should be 2 incomplete tasks");
}

#[test]
fn test_task_names() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/data/tasks.txt");

    let input = fs::read_to_string(path)
        .expect("Failed to read tasks.txt");

    let tasks = parse_input(&input);

    let expected_names = vec![
        "This is a test",
        "This is finished",
        "This is not",
        "Neither is this",
    ];

    // Compare each task's name with the expected name
    for (task, &expected_name) in tasks.iter().zip(expected_names.iter()) {
        assert_eq!(task.name, expected_name, "Task name does not match expected value");
    }
    
    assert_eq!(tasks.len(), expected_names.len(), "Number of parsed tasks does not match expected number");
}

#[test]
fn test_tasks_details() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/data/task_dates.txt");

    let input = fs::read_to_string(path)
        .expect("Failed to read tasks.txt");

    let tasks = parse_input(&input);

    let expected_details = vec![
        ("This a task with no due data", None),
        ("This is another one, but with a due date", Some(NaiveDate::from_ymd(2025, 7, 14))),
        ("This one is overdue", Some(NaiveDate::from_ymd(2021, 7, 14))),
        ("This one has an invalid due date", None), // Assuming your parsing correctly handles invalid dates
    ];

    // Compare each task's name and due date with the expected values
    assert_eq!(tasks.len(), expected_details.len(), "Number of parsed tasks does not match expected number");
    for (task, &(expected_name, expected_due)) in tasks.iter().zip(expected_details.iter()) {
        assert_eq!(task.name, expected_name, "Task name does not match expected value");
        assert_eq!(task.due, expected_due, "Task due date does not match expected value");
    }
}
