# CLI Todo List Tool in Rust

Welcome to my **CLI Todo List Tool** project! This is a simple command-line application built in Rust to help me learn and practice the language. The tool allows users to manage tasks (add, list, complete, and delete) and saves them to a JSON file for persistence.

This project is part of my journey to learn Rust, and it has been a great way to explore concepts like ownership, error handling, and working with external libraries.

---

## Features
- **Add a task**: Add a new task with a description.
- **List tasks**: View all tasks with their completion status.
- **Complete a task**: Mark a task as completed.
- **Delete a task**: Remove a task from the list.
- **Persistent storage**: Tasks are saved to a `tasks.json` file and loaded when the program starts.

---

## Libraries Used
- **[`clap`](https://crates.io/crates/clap)**: For parsing command-line arguments.
- **[`serde`](https://crates.io/crates/serde)**: For serializing and deserializing tasks to/from JSON.
- **[`serde_json`](https://crates.io/crates/serde_json)**: For working with JSON data format.
- **[`std::fs`](https://doc.rust-lang.org/std/fs/)**: For file I/O operations (saving and loading tasks).

---

## Lessons Learned
- **Rust Basics**: Reinforced my understanding of Rust's core concepts like ownership, borrowing, and lifetimes.
- **Error Handling**: Learned how to use `Result` and `?` for robust error handling.
- **CLI Development**: Gained experience in building command-line tools using `clap`.
- **Serialization**: Explored how to serialize and deserialize data using `serde` and `serde_json`.
- **File I/O**: Practiced reading from and writing to files using Rust's standard library.

---

## How to Use
1. Clone the repository:
   ```bash
   git clone https://github.com/SouvickSarkar20/CLI-Tool.git
   cd CLI-Tool
