mod user_input;
mod todo_table;
mod task_status;

use todo_table::ToDoTable;
use user_input::{get_task_input, get_task_update_input};

fn main() {
    let mut todo_table = ToDoTable::new();

    loop {
        println!("1: Add task, 2: Update task, 3: Display tasks, 4: Exit");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                let (task, status) = get_task_input();
                todo_table.add_task(task, status);
            },
            "2" => {
                match get_task_update_input(&todo_table.tasks) {
                    Some((name, new_status)) => todo_table.update_task_status(&name, new_status),
                    None => println!("Task update was cancelled or invalid input was provided."),
                    }
            },
            "3" => {
                todo_table.display();
            },
            "4" => break,
            _ => println!("Invalid choice."),
        }
    }
}
