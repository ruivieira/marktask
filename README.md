[![Rust](https://github.com/ruivieira/marktask/actions/workflows/rust.yaml/badge.svg)](https://github.com/ruivieira/marktask/actions/workflows/rust.yaml) [![](https://img.shields.io/crates/v/marktask.svg)](https://crates.io/crates/marktask)
# marktask

`marktask` is a command-line interface (CLI) application designed for parsing and manipulating Markdown tasks.
It allows users to filter, extract, and transform task information seamlessly within their shell environment.

## Features

- Parse Markdown-formatted tasks.
- Extract tasks with due dates, scheduled dates, and start dates.
- Filter tasks based on completion status.
- Filter out or include overdue tasks with the `--overdue` option.
- Output tasks in plain text or JSON format for easy consumption by other tools.
- Filter tasks within a specific date range using `--from` and `--to` options.

## Installation

```sh
cargo build --release
mv target/release/marktask /usr/local/bin
```

## Usage

`marktask` can be used with pipes and with other commands in a shell environment. 
Below are some examples to get you started:

### Display Tasks from a File

```sh
marktask < tasks.md  # or
cat tasks.md | marktask
```

### Filter Tasks Containing Specific Text

```sh
cat tasks.md | marktask | grep "Project A"
```

This command sequence reads tasks from `tasks.md`, parses them, and then uses `grep` to filter tasks related to "Project A".

### Convert Tasks to JSON

For integration with other tools that consume JSON, `marktask` can output tasks in JSON format:

```sh
cat tasks.md | marktask --json
```

### Filter Overdue Tasks

To exclude overdue tasks from the output, use the `--overdue=false` option. 
By default, all tasks, including overdue ones, are shown:

```sh
cat tasks.md | marktask --overdue=false
```

### Filter Tasks by Date Range

To include tasks starting from a specific date:

```sh
cat tasks.md | marktask --from 2024-01-01
```

To include tasks up to a specific date:

```sh
cat tasks.md | marktask --to 2024-01-31
```

To include tasks within a specific date range:

```sh
cat tasks.md | marktask --from 2024-01-01 --to 2024-01-31
```

This filters tasks to include only those that fall within the specified date range.

## License

This project is licensed under the Apache License Version 2.0 - see the [LICENSE](./LICENSE) file for details.