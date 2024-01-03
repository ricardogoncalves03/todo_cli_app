mod storage;
mod task_status;
mod todo_table;
mod user_input;

use storage::{load_tasks, save_tasks};
use todo_table::ToDoTable;
use user_input::{get_task_input, get_task_name_input, get_task_update_input};

fn main() {
    let mut todo_table = ToDoTable::new();

    todo_table.tasks = load_tasks();
    todo_table.rebuild_table();

    loop {
        println!("1: Add task, 2: Update task, 3: Display tasks, 4: Delete task, 5: Exit");
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                let (task, status) = get_task_input();
                todo_table.add_task(task, status);
                save_tasks(&todo_table.tasks);
            }
            "2" => match get_task_update_input(&todo_table.tasks) {
                Some((name, new_status)) => {
                    todo_table.update_task_status(&name, new_status);
                    save_tasks(&todo_table.tasks);
                }
                None => println!("Task update was cancelled or invalid input was provided."),
            },
            "3" => {
                todo_table.display();
            }
            "4" => {
                if let Some(task_name) = get_task_name_input(&todo_table.tasks) {
                    todo_table.delete_task(&task_name);
                    save_tasks(&todo_table.tasks);
                } else {
                    println!("Task deletion cancelled.");
                }
            }
            "5" => break,
            _ => println!("Invalid choice."),
        }
    }
}
