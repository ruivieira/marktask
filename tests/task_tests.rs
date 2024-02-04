use std::fs;
use std::path::PathBuf;
use marktask::parse_input;
use marktask::Task;

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
