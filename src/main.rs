use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const TODO_FILE: &str = "todo.json";

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple command-line todo application")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// The task description
        #[arg(value_name = "TASK")]
        task: String,
    },
    /// List all tasks
    List,
    /// Remove a task by index
    Remove {
        /// The index of the task to remove (1-based)
        #[arg(value_name = "INDEX")]
        index: usize,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    description: String,
}

impl Task {
    fn new(description: String) -> Self {
        Task { description }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn remove_task(&mut self, index: usize) -> Result<(), String> {
        if index == 0 || index > self.tasks.len() {
            return Err(format!(
                "Invalid index {}. Please use a number between 1 and {}",
                index,
                self.tasks.len()
            ));
        }
        self.tasks.remove(index - 1);
        Ok(())
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
            return;
        }

        println!("Your tasks:");
        for (i, task) in self.tasks.iter().enumerate() {
            println!("{}. {}", i + 1, task.description);
        }
    }
}

fn load_todos() -> Result<TodoList, Box<dyn std::error::Error>> {
    if !Path::new(TODO_FILE).exists() {
        // If file doesn't exist, return a new empty todo list
        return Ok(TodoList::new());
    }

    let contents = fs::read_to_string(TODO_FILE)?;

    // Handle empty file
    if contents.trim().is_empty() {
        return Ok(TodoList::new());
    }

    // Try to parse JSON, handle invalid JSON gracefully
    match serde_json::from_str(&contents) {
        Ok(todo_list) => Ok(todo_list),
        Err(_) => {
            eprintln!("Warning: Invalid JSON in todo file. Starting with empty list.");
            Ok(TodoList::new())
        }
    }
}

fn save_todos(todo_list: &TodoList) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(todo_list)?;
    fs::write(TODO_FILE, json)?;
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    // Load existing todos
    let mut todo_list = match load_todos() {
        Ok(list) => list,
        Err(e) => {
            eprintln!("Error loading todos: {}", e);
            std::process::exit(1);
        }
    };

    // Execute the command
    match cli.command {
        Commands::Add { task } => {
            todo_list.add_task(Task::new(task.clone()));

            if let Err(e) = save_todos(&todo_list) {
                eprintln!("Error saving task: {}", e);
                std::process::exit(1);
            }

            println!("Added task: \"{}\"", task);
        }
        Commands::List => {
            todo_list.list_tasks();
        }
        Commands::Remove { index } => {
            match todo_list.remove_task(index) {
                Ok(()) => {
                    if let Err(e) = save_todos(&todo_list) {
                        eprintln!("Error saving after removal: {}", e);
                        std::process::exit(1);
                    }
                    println!("Removed task at index {}", index);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}