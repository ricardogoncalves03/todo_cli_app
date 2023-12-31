mod todo_table;
mod task_status;

use todo_table::ToDoTable;
use task_status::TaskStatus;

fn main() {

    let mut todo_table = ToDoTable::new();

    todo_table.add_task("Learn Rust".to_string(), TaskStatus::Backlog);
    todo_table.add_task("Grocery Shopping".to_string(), TaskStatus::ToDo);
    
    todo_table.display();

    todo_table.update_task_status("Learn Rust", TaskStatus::ToDo);

    todo_table.display();

}
