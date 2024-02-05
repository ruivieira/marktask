use chrono::{Duration, Local, NaiveDate};
use marktask::dates::parse_date_arg;
use marktask::Task;
use marktask::{
    parse_input, parse_priority, DateRangeFilter, FilterPipeline, OverdueFilter, Priority,
};
use std::fs;
use std::path::PathBuf;

#[test]
fn test_task_completion() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/data/tasks.txt");

    let input = fs::read_to_string(path).expect("Failed to read tasks.txt");

    let tasks = parse_input(&input);

    let completed_tasks: Vec<&Task> = tasks.iter().filter(|t| t.completed).collect();
    let incomplete_tasks: Vec<&Task> = tasks.iter().filter(|t| !t.completed).collect();

    assert_eq!(
        completed_tasks.len(),
        1,
        "There should be 2 completed tasks"
    );
    assert_eq!(
        incomplete_tasks.len(),
        3,
        "There should be 2 incomplete tasks"
    );
}

#[test]
fn test_task_names() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/data/tasks.txt");

    let input = fs::read_to_string(path).expect("Failed to read tasks.txt");

    let tasks = parse_input(&input);

    let expected_names = vec![
        "This is a test",
        "This is finished",
        "This is not",
        "Neither is this",
    ];

    // Compare each task's name with the expected name
    for (task, &expected_name) in tasks.iter().zip(expected_names.iter()) {
        assert_eq!(
            task.name, expected_name,
            "Task name does not match expected value"
        );
    }

    assert_eq!(
        tasks.len(),
        expected_names.len(),
        "Number of parsed tasks does not match expected number"
    );
}

#[test]
fn test_tasks_details() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/data/task_dates.txt");

    let input = fs::read_to_string(path).expect("Failed to read task_dates.txt");

    let tasks = parse_input(&input);

    let expected_details = vec![
        ("This a task with no due data", None, None, None),
        (
            "This is another one, but with a due date",
            Some(NaiveDate::from_ymd(2025, 7, 14)),
            None,
            None,
        ),
        (
            "This one is overdue",
            Some(NaiveDate::from_ymd(2021, 7, 14)),
            None,
            None,
        ),
        ("This one has an invalid due date", None, None, None),
        (
            "This has both a due and scheduled date",
            Some(NaiveDate::from_ymd(2025, 7, 14)),
            Some(NaiveDate::from_ymd(2025, 7, 14)),
            None,
        ),
        (
            "This has a wrong scheduled date",
            Some(NaiveDate::from_ymd(2025, 7, 14)),
            None,
            None,
        ),
        ("This has both dates wrong", None, None, None),
        (
            "This has just the due date wrong",
            None,
            Some(NaiveDate::from_ymd(2025, 7, 14)),
            None,
        ),
        (
            "This one has just a scheduled date (but wrong)",
            None,
            None,
            None,
        ),
        (
            "This one has just a scheduled date",
            None,
            Some(NaiveDate::from_ymd(2025, 7, 14)),
            None,
        ),
        (
            "To start",
            None,
            None,
            Some(NaiveDate::from_ymd(2024, 2, 7)),
        ),
        (
            "Start and due",
            Some(NaiveDate::from_ymd(2025, 7, 14)),
            None,
            Some(NaiveDate::from_ymd(2024, 2, 7)),
        ),
        (
            "Start and scheduled",
            None,
            Some(NaiveDate::from_ymd(2025, 7, 15)),
            Some(NaiveDate::from_ymd(2024, 2, 7)),
        ),
        (
            "All dates present",
            Some(NaiveDate::from_ymd(2025, 7, 14)),
            Some(NaiveDate::from_ymd(2025, 7, 15)),
            Some(NaiveDate::from_ymd(2024, 2, 7)),
        ),
        ("Start with invalid date", None, None, None),
    ];

    assert_eq!(
        tasks.len(),
        expected_details.len(),
        "Number of parsed tasks does not match expected number"
    );

    for (task, &(expected_name, expected_due, expected_scheduled, expected_start)) in
        tasks.iter().zip(expected_details.iter())
    {
        assert_eq!(
            task.name, expected_name,
            "Task name does not match expected value"
        );
        assert_eq!(
            task.due, expected_due,
            "Task due date does not match expected value"
        );
        assert_eq!(
            task.scheduled, expected_scheduled,
            "Task scheduled date does not match expected value"
        );
        assert_eq!(
            task.start, expected_start,
            "Task start date does not match expected value"
        );
    }
}

#[test]
fn test_overdue_tasks() {
    // Generate dynamic dates for the tasks
    let today = Local::today().naive_local();
    let yesterday = today - Duration::days(1);
    let tomorrow = today + Duration::days(1);

    // Task strings with dynamic dates
    let input = format!(
        "- [ ] Task due today 📅 {}\n\
             - [ ] Overdue task 📅 {}\n\
             - [ ] Not overdue task 📅 {}",
        today, yesterday, tomorrow
    );

    // Parse the input string to tasks
    let tasks = parse_input(&input);

    // Expected overdue statuses
    let expected_overdue = vec![
        false, // Task due today is not considered overdue
        true,  // Task due yesterday is overdue
        false, // Task due tomorrow is not overdue
    ];

    // Compare each task's overdue status with the expected status
    assert_eq!(
        tasks.len(),
        expected_overdue.len(),
        "Number of tasks parsed does not match expected number"
    );
    for (task, &expected) in tasks.iter().zip(expected_overdue.iter()) {
        assert_eq!(
            task.overdue, expected,
            "Task overdue status does not match expected value"
        );
    }
}

#[test]
fn test_overdue_filter_pipeline() {
    // Create a date for today and a date in the past
    let today = Local::today().naive_local();
    let past_date = Local::today().naive_local() - chrono::Duration::days(1);

    // Create sample tasks
    let tasks = vec![
        Task {
            name: "Task due today".to_string(),
            completed: false,
            due: Some(today),
            overdue: false,
            start: None,
            scheduled: None,
            priority: Priority::None,
        },
        Task {
            name: "Overdue task".to_string(),
            completed: false,
            due: Some(past_date),
            overdue: true,
            start: None,
            scheduled: None,
            priority: Priority::None,
        },
        Task {
            name: "No due date task".to_string(),
            completed: false,
            due: None,
            overdue: false,
            start: None,
            scheduled: None,
            priority: Priority::None,
        },
    ];

    // Initialize the filter pipeline and add the OverdueFilter
    let mut pipeline = FilterPipeline::new();
    pipeline.add_filter(Box::new(OverdueFilter {
        show_overdue: false,
    }));

    // Apply the pipeline filters
    let task_refs: Vec<&Task> = tasks.iter().collect();
    let filtered_tasks = pipeline.apply(task_refs);

    // Assert that the filtered tasks do not include the overdue task
    assert_eq!(
        filtered_tasks.len(),
        2,
        "Filtered tasks should not include overdue tasks"
    );
    assert!(
        filtered_tasks.iter().all(|&task| !task.overdue),
        "All filtered tasks should not be overdue"
    );
}

#[test]
fn test_task_priorities() {
    // Example task descriptions with various priority signifiers
    let tasks_data = vec![
        ("Do something important 🔺", Priority::Highest),
        ("Do something else ⏫", Priority::High),
        ("Regular task 🔼", Priority::Medium),
        ("Maybe do this sometime 🔽", Priority::Low),
        ("Not important ⏬", Priority::Lowest),
        ("An ordinary task", Priority::None), // No signifier indicates no specific priority
    ];

    // Iterate over the task descriptions and their expected priorities
    for (description, expected_priority) in tasks_data {
        let d = description.to_string();
        let task = Task {
            name: d,
            completed: false,
            due: Some(NaiveDate::from_ymd(2022, 1, 1)), // Dummy date
            overdue: false,
            scheduled: None,
            start: None,
            priority: parse_priority(description).1,
        };

        // Assert that the parsed priority matches the expected priority
        assert_eq!(
            task.priority, expected_priority,
            "Priority did not match for task description: {}",
            description
        );
    }
}

#[test]
fn test_date_range_filtering() {
    // Define a set of tasks with various due dates
    let base_date = NaiveDate::from_ymd(2024, 1, 1);
    let tasks = vec![
        Task {
            name: "Task 1".to_string(),
            completed: false,
            due: Some(base_date),
            overdue: true,
            scheduled: None,
            start: None,
            priority: Priority::None,
        },
        Task {
            name: "Task 2".to_string(),
            completed: false,
            due: Some(base_date + Duration::days(5)),
            overdue: true,
            scheduled: None,
            start: None,
            priority: Priority::None,
        },
        Task {
            name: "Task 3".to_string(),
            completed: false,
            due: Some(base_date + Duration::days(10)),
            overdue: true,
            scheduled: None,
            start: None,
            priority: Priority::None,
        },
        Task {
            name: "Task without date".to_string(),
            completed: false,
            due: None,
            overdue: false,
            scheduled: None,
            start: None,
            priority: Priority::None,
        },
    ];

    // Define test cases
    let test_cases = vec![
        (
            Some(base_date),
            Some(base_date + Duration::days(10)),
            3,
            "Both 'from' and 'to' specified",
        ),
        (
            Some(base_date + Duration::days(5)),
            None,
            2,
            "'From' specified only",
        ),
        (
            None,
            Some(base_date + Duration::days(5)),
            2,
            "'To' specified only",
        ),
        (None, None, 4, "Neither 'from' nor 'to' specified"),
    ];

    for (from_date, to_date, expected_count, case_description) in test_cases {
        let mut pipeline = FilterPipeline::new();
        pipeline.add_filter(Box::new(DateRangeFilter { from_date, to_date }));

        let filtered_tasks = pipeline.apply(tasks.iter().collect());

        assert_eq!(
            filtered_tasks.len(),
            expected_count,
            "Case '{}': Expected {} tasks, found {}",
            case_description,
            expected_count,
            filtered_tasks.len()
        );
    }
}

#[test]
fn test_relative_date_range_filtering() {
    let today = Local::today().naive_local();
    let tasks = vec![
        Task {
            name: "Task due today".to_string(),
            completed: false,
            due: Some(today),
            overdue: false,
            scheduled: None,
            start: None,
            priority: Priority::None,
        },
        Task {
            name: "Task due in 5 days".to_string(),
            completed: false,
            due: Some(today + Duration::days(5)),
            overdue: false,
            scheduled: None,
            start: None,
            priority: Priority::None,
        },
        Task {
            name: "Task due in 10 days".to_string(),
            completed: false,
            due: Some(today + Duration::days(10)),
            overdue: false,
            scheduled: None,
            start: None,
            priority: Priority::None,
        },
        Task {
            name: "Task without date".to_string(),
            completed: false,
            due: None,
            overdue: false,
            scheduled: None,
            start: None,
            priority: Priority::None,
        },
    ];

    // Define test cases with relative dates
    let test_cases = vec![
        (
            parse_date_arg(Some("-1d")),
            parse_date_arg(Some("+2w")),
            3,
            "Tasks within last day to next week",
        ),
        (
            None,
            parse_date_arg(Some("+1w")),
            2,
            "Tasks up to one week from today",
        ),
        (
            parse_date_arg(Some("-1d")),
            None,
            3,
            "Tasks from yesterday onwards",
        ),
        (None, None, 4, "No date filtering"),
    ];

    for (from_date, to_date, expected_count, case_description) in test_cases {
        let mut pipeline = FilterPipeline::new();
        let date_range_filter = DateRangeFilter { from_date, to_date };
        pipeline.add_filter(Box::new(date_range_filter));

        let filtered_tasks = pipeline.apply(tasks.iter().collect());

        assert_eq!(
            filtered_tasks.len(),
            expected_count,
            "Case '{}': Expected {} tasks, found {}",
            case_description,
            expected_count,
            filtered_tasks.len()
        );
    }
}
