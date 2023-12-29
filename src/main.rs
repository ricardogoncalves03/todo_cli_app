mod todo_table;
mod task_group;

use todo_table::ToDoTable;
// use comfy_table::Table;
// use cli_display_table::{add_row, setup_to_do_table};

fn main() {
    let mut todo_table = ToDoTable::new();
    todo_table.add_to_backlog("Fix bug in code".to_string());
    todo_table.add_to_todo("Write documentation".to_string());
    todo_table.add_to_done("Deploy app".to_string());

    // Display the table
    todo_table.display();
}
