use clap::{Arg, Command}; // Importing clap for command-line argument parsing
use serde::{Deserialize, Serialize}; // Importing serde for serializing and deserializing JSON
use std::{fs, process}; // Importing fs for file system operations and process for handling errors

// Struct that represents a Task
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    description: String, // Description of the task
    completed: bool, // Task completion status
}

// Function to load tasks from a JSON file
fn load_tasks() -> Vec<Task> {
    // Try to read the tasks from a file named "tasks.json"
    if let Ok(tasks_json) = fs::read_to_string("tasks.json") {
        // Try to deserialize the JSON string into a vector of tasks
        if let Ok(tasks) = serde_json::from_str(&tasks_json) {
            return tasks; // Return the loaded tasks
        }
    }
    // Return an empty vector if tasks could not be loaded
    vec![]
}

// Function to save tasks to a JSON file
fn save_tasks(tasks: &Vec<Task>) {
    // Serialize the tasks vector into a pretty JSON string
    let tasks_json = serde_json::to_string_pretty(tasks).expect("Error serializing tasks.");
    // Write the serialized JSON to the "tasks.json" file
    fs::write("tasks.json", tasks_json).expect("Error saving tasks.");
}

// Function to list all tasks, optionally filtered by a status (e.g., pending or completed)
fn list_tasks(tasks: &Vec<Task>, filter: Option<String>) {
    let filter = filter.unwrap_or_else(|| String::from("")); // If no filter is provided, use an empty string as default

    // Filter tasks based on the filter value ('pending', 'completed', or none)
    let filtered_tasks: Vec<&Task> = tasks.iter()
        .filter(|t| {
            match filter.as_str() {
                "pending" => !t.completed, // Only tasks that are not completed
                "completed" => t.completed, // Only tasks that are completed
                _ => true, // Include all tasks if no valid filter is provided
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
            println!("{}. {} ({})", index + 1, task.description, status); // Print task number, description, and status
        }
    }
}

// Function to add a new task
fn add_task(tasks: &mut Vec<Task>, description: String) {
    // Create a new task with the provided description and set it as not completed
    let new_task = Task { description: description.clone(), completed: false };
    tasks.push(new_task); // Add the new task to the task list
    save_tasks(tasks); // Save the updated task list to the file
    println!("Task '{}' added successfully!", description); // Print success message
}

// Function to mark a task as completed based on its ID
fn mark_task_as_completed(tasks: &mut Vec<Task>, id: usize) {
    // Check if the ID is valid (greater than 0 and less than or equal to the number of tasks)
    if id > 0 && id <= tasks.len() {
        tasks[id - 1].completed = true; // Set the task's status to completed
        save_tasks(tasks); // Save the updated task list
        println!("Task {} marked as completed!", id); // Print success message
    } else {
        println!("Invalid task ID."); // Print error message if the ID is invalid
    }
}

// Function to remove a task based on its ID
fn remove_task(tasks: &mut Vec<Task>, id: usize) {
    // Check if the ID is valid
    if id > 0 && id <= tasks.len() {
        let removed_task = tasks.remove(id - 1); // Remove the task by ID
        save_tasks(tasks); // Save the updated task list
        println!("Task '{}' removed successfully!", removed_task.description); // Print success message
    } else {
        println!("Invalid task ID."); // Print error message if the ID is invalid
    }
}

// Main function that processes command-line arguments and manages tasks
fn main() {
    println!("{{\n");

    // Parsing command-line arguments using clap
    let matches = Command::new("Task Manager CLI")
        .version("1.0")
        .about("CLI to manage pending tasks") // Brief description of the CLI
        .after_help("}") // Formatting for the help message
        .subcommand(Command::new("add") // Subcommand to add a new task
            .about("Adds a new task")
            .after_help("}")
            .arg(Arg::new("description") // Argument for the task description
                .help("Task description")
                .required(true) // The description is required
                .index(1))) // The description is the first positional argument
        .subcommand(Command::new("list") // Subcommand to list tasks
            .about("Lists the tasks")
            .after_help("}")
            .arg(Arg::new("filter") // Optional filter argument for filtering by status
                .help("Filters by 'pending' or 'completed'")
                .long("filter") // --filter flag
                .num_args(1)))  // Indicating it accepts one value for filtering
        .subcommand(Command::new("complete") // Subcommand to mark a task as completed
            .about("Marks a task as completed")
            .after_help("}")
            .arg(Arg::new("id") // Argument for the task ID
                .help("Task ID")
                .required(true) // ID is required
                .index(1))) // The ID is the first positional argument
        .subcommand(Command::new("remove") // Subcommand to remove a task
            .about("Removes a task")
            .after_help("}")
            .arg(Arg::new("id") // Argument for the task ID
                .help("Task ID")
                .required(true) // ID is required
                .index(1))) // The ID is the first positional argument
        .get_matches(); // Get the parsed matches

    let mut tasks = load_tasks(); // Load the current tasks from the file

    // Match the subcommand and execute the corresponding functionality
    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let description = sub_m.get_one::<String>("description").unwrap().to_string();
            add_task(&mut tasks, description); // Add a new task
        }
        Some(("list", sub_m)) => {
            let filter = sub_m.get_one::<String>("filter").map(|f| f.to_string());
            list_tasks(&tasks, filter); // List tasks with an optional filter
        }
        Some(("complete", sub_m)) => {
            let id: usize = sub_m.get_one::<String>("id")
                .unwrap()
                .parse()
                .unwrap_or_else(|_| {
                    eprintln!("Invalid ID."); // Error handling for invalid ID
                    process::exit(1); // Exit with an error code
                });
            mark_task_as_completed(&mut tasks, id); // Mark the task as completed
        }
        Some(("remove", sub_m)) => {
            let id: usize = sub_m.get_one::<String>("id")
                .unwrap()
                .parse()
                .unwrap_or_else(|_| {
                    eprintln!("Invalid ID."); // Error handling for invalid ID
                    process::exit(1); // Exit with an error code
                });
            remove_task(&mut tasks, id); // Remove the task
        }
        _ => {
            eprintln!("Invalid command."); // Error handling for invalid command
            process::exit(1); // Exit with an error code
        }
    }

    println!("\n}}"); // Closing brackets for formatting
}

