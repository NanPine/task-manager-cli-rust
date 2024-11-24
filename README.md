# Task Manager CLI

A simple command-line interface (CLI) application built with Rust for managing tasks.

## Features
- Add tasks
- List tasks
- Mark tasks as completed
- Remove tasks

## Usage

- `cargo run add <task>` - Add a new task.
- `cargo run list` - List all tasks.
- `cargo run list --filter <status>` - List tasks with a specific status (`pending` or `completed`).
- `cargo run complete <task_id>` - Mark a task as completed.
- `cargo run remove <task_id>` - Remove a task.

## Installation

### Prerequisites
- Rust: https://www.rust-lang.org/tools/install

### Install Dependencies
Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/NanPine/task-manager-cli-rust.git
cd task-manager-cli-rust
cargo build --release

