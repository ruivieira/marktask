[![Rust](https://github.com/ruivieira/marktask/actions/workflows/rust.yaml/badge.svg)](https://github.com/ruivieira/marktask/actions/workflows/rust.yaml)
# marktask

`marktask` is a command-line interface (CLI) application designed for parsing and manipulating Markdown tasks. 
It allows users to filter, extract, and transform task information seamlessly within their shell environment.

## Features

- Parse Markdown-formatted tasks
- Extract tasks with due dates, scheduled dates, and start dates
- Filter tasks based on completion status
- Output tasks in plain text or JSON format for easy consumption by other tools

## Installation

```sh
cargo build
mv marktask /usr/local/bin
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
cat tasks.md | taskparser | grep "Project A"
```

This command sequence reads tasks from `tasks.md`, parses them, and then uses `grep` to filter tasks related to "Project A".

### Convert Tasks to JSON

For integration with other tools that consume JSON, TaskParser can output tasks in JSON format:

```sh
cat tasks.md | taskparser --json
```

## License

This project is licensed under the Apache License Version 2.0 - see the [LICENSE](./LICENSE) file for details.