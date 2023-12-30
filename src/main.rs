mod todo_table;
mod task_status;

use std::ptr::eq;

use todo_table::ToDoTable;
use task_status::TaskStatus;
use comfy_table::Table;
// use cli_display_table::{add_row, setup_to_do_table};

fn main() {

    let mut todo_table = ToDoTable::new();

    todo_table.add_task("Learn Rust".to_string(), TaskStatus::Backlog);
    todo_table.add_task("Grocery Shopping".to_string(), TaskStatus::ToDo);
    
    todo_table.display();

    todo_table.update_task_status("Learn Rust".to_string(), TaskStatus::ToDo);

    // todo_table.display();

}
