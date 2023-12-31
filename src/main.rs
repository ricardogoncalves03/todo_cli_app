mod task_status;
mod todo_table;

use task_status::TaskStatus;
use todo_table::ToDoTable;

fn main() {
    let mut todo_table = ToDoTable::new();

    todo_table.add_task("Learn Rust".to_string(), TaskStatus::Backlog);
    todo_table.add_task("Grocery Shopping".to_string(), TaskStatus::ToDo);

    todo_table.display();

    todo_table.update_task_status("Learn Rust", TaskStatus::ToDo);

    todo_table.display();
}
