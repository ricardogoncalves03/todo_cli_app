use std::io;
use crate::task_status::TaskStatus;
use std::collections::HashMap;

pub fn get_task_input() -> (String, TaskStatus) {
    let mut task_name = String::new();
    let mut task_status = String::new();

    println!("Enter task name:");
    io::stdin().read_line(&mut task_name).expect("Failed to read line");

    println!("Enter task status (1: Backlog, 2: ToDo, 3: Done):");
    io::stdin().read_line(&mut task_status).expect("Failed to read line");

    let status = match task_status.trim() {
        "1" => TaskStatus::Backlog,
        "2" => TaskStatus::ToDo,
        "3" => TaskStatus::Done,
        _ => {
            println!("Invalid status, defaulting to Backlog");
            TaskStatus::Backlog
        }
    };

    (task_name.trim().to_string(), status)
}

pub fn get_task_update_input(tasks: &HashMap<String, TaskStatus>) -> Option<(String, TaskStatus)> {
    if tasks.is_empty() {
        println!("No tasks available to update.");
        return None;
    }

    // Display all tasks with their names
    println!("Available tasks:");
    for (task, status) in tasks {
        println!("{} (current status: {})", task, status);
    }

    let mut task_name = String::new();
    println!("Enter the name of the task to update:");
    io::stdin().read_line(&mut task_name).expect("Failed to read line");
    let task_name = task_name.trim();

    // Check if the task exists
    if !tasks.contains_key(task_name) {
        println!("Task not found.");
        return None;
    }

    // Get new status
    let mut status_str = String::new();
    println!("Enter the new task status (1: Backlog, 2: ToDo, 3: Done):");
    io::stdin().read_line(&mut status_str).expect("Failed to read line");

    let status = match status_str.trim() {
        "1" => TaskStatus::Backlog,
        "2" => TaskStatus::ToDo,
        "3" => TaskStatus::Done,
        _ => {
            println!("Invalid status.");
            return None;
        }
    };

    Some((task_name.to_string(), status))
}

