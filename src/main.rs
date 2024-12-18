use clap::{Arg, Command}; // Importing clap for command-line argument parsing
use serde::{Deserialize, Serialize}; // Importing serde for serializing and deserializing JSON
use std::{fs, process}; // Importing fs for file system operations and process for handling errors

// Struct that represents a Task
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    description: String,
    completed: bool,
}

// Function to load tasks from a JSON file
fn load_tasks() -> Vec<Task> {
    if let Ok(tasks_json) = fs::read_to_string("tasks.json") {
        if let Ok(tasks) = serde_json::from_str(&tasks_json) {
            return tasks;
        }
    }
    vec![]
}

// Function to save tasks to a JSON file
fn save_tasks(tasks: &Vec<Task>) {
    let tasks_json = serde_json::to_string_pretty(tasks).expect("Error serializing tasks.");
    fs::write("tasks.json", tasks_json).expect("Error saving tasks.");
}

// Function to list all tasks, optionally filtered by a status (e.g., pending or completed)
fn list_tasks(tasks: &Vec<Task>, filter: Option<String>) {
    let filter = filter.unwrap_or_else(|| String::from(""));

    // Filter tasks based on the filter value ('pending', 'completed', or none)
    let filtered_tasks: Vec<&Task> = tasks.iter()
        .filter(|t| {
            match filter.as_str() {
                "pending" => !t.completed,
                "completed" => t.completed,
                _ => true,
            }
        })
        .collect();

    // Print the filtered tasks, or a message if no tasks were found
    if filtered_tasks.is_empty() {
        println!("No tasks found.");
    } else {
        // Loop through each filtered task and print its description and status
        for (index, task) in filtered_tasks.iter().enumerate() {
            let status = if task.completed { "Completed" } else { "Pending" };
            println!("{}. {} ({})", index + 1, task.description, status);
        }
    }
}

// Function to add a new task
fn add_task(tasks: &mut Vec<Task>, description: String) {
    let new_task = Task { description: description.clone(), completed: false };
    tasks.push(new_task);
    save_tasks(tasks);
    println!("Task '{}' added successfully!", description);
}

// Function to mark a task as completed based on its ID
fn mark_task_as_completed(tasks: &mut Vec<Task>, id: usize) {
    if id > 0 && id <= tasks.len() {
        tasks[id - 1].completed = true;
        save_tasks(tasks);
        println!("Task {} marked as completed!", id);
    } else {
        println!("Invalid task ID.");
    }
}

// Function to remove a task based on its ID
fn remove_task(tasks: &mut Vec<Task>, id: usize) {
    if id > 0 && id <= tasks.len() {
        let removed_task = tasks.remove(id - 1);
        save_tasks(tasks);
        println!("Task '{}' removed successfully!", removed_task.description);
    } else {
        println!("Invalid task ID.");
    }
}

// Main function that processes command-line arguments and manages tasks
fn main() {
    println!("{{\n");

    // Parsing command-line arguments using clap
    let matches = Command::new("Task Manager CLI")
        .version("1.0")
        .about("CLI to manage pending tasks")
        .after_help("}")
        .subcommand(Command::new("add")
            .about("Adds a new task")
            .after_help("}")
            .arg(Arg::new("description")
                .help("Task description")
                .required(true)
                .index(1)))
        .subcommand(Command::new("list")
            .about("Lists the tasks")
            .after_help("}")
            .arg(Arg::new("filter")
                .help("Filters by 'pending' or 'completed'")
                .long("filter")
                .num_args(1)))
        .subcommand(Command::new("complete")
            .about("Marks a task as completed")
            .after_help("}")
            .arg(Arg::new("id")
                .help("Task ID")
                .required(true)
                .index(1)))
        .subcommand(Command::new("remove")
            .about("Removes a task")
            .after_help("}")
            .arg(Arg::new("id")
                .help("Task ID")
                .required(true)
                .index(1)))
        .get_matches();

    let mut tasks = load_tasks();

    // Match the subcommand and execute the corresponding functionality
    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let description = sub_m.get_one::<String>("description").unwrap().to_string();
            add_task(&mut tasks, description);
        }
        Some(("list", sub_m)) => {
            let filter = sub_m.get_one::<String>("filter").map(|f| f.to_string());
            list_tasks(&tasks, filter);
        }
        Some(("complete", sub_m)) => {
            let id: usize = sub_m.get_one::<String>("id")
                .unwrap()
                .parse()
                .unwrap_or_else(|_| {
                    eprintln!("Invalid ID.");
                    process::exit(1);
                });
            mark_task_as_completed(&mut tasks, id);
        }
        Some(("remove", sub_m)) => {
            let id: usize = sub_m.get_one::<String>("id")
                .unwrap()
                .parse()
                .unwrap_or_else(|_| {
                    eprintln!("Invalid ID.");
                    process::exit(1);
                });
            remove_task(&mut tasks, id);
        }
        _ => {
            eprintln!("Invalid command.");
            process::exit(1);
        }
    }

    println!("\n}}");
}

