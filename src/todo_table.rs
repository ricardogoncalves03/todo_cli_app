use std::collections::HashMap;
use crate::task_status::TaskStatus;
use comfy_table::{Table, ContentArrangement, Cell, CellAlignment};

pub struct ToDoTable {
    table: Table,
    tasks: HashMap<String, TaskStatus>,
}

impl ToDoTable {
    pub fn new() -> Self {
        let mut table = Table::new();
        table
            .set_header(vec![
                TaskStatus::Backlog.to_string(),
                TaskStatus::ToDo.to_string(),
                TaskStatus::Done.to_string(),
                ])
            .set_content_arrangement(ContentArrangement::DynamicFullWidth);

        Self {
            table,
            tasks: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, task: String, status: TaskStatus) {
        // Update the tasks HashMap
        self.tasks.insert(task.clone(), status);

        // Decide where to place the task in the table based on its status
        match status {
            TaskStatus::Backlog => self.table.add_row(vec![task, "".to_string(), "".to_string()]),
            TaskStatus::ToDo => self.table.add_row(vec!["".to_string(), task, "".to_string(),]),
            TaskStatus::Done => self.table.add_row(vec!["".to_string(), "".to_string(), task]),
        };
    }

    // This method can be extended or modified to handle task updates and re-render the table
    pub fn update_task_status(&mut self, task: String, new_status: TaskStatus) {
        self.tasks.remove(&task);
        self.add_task(task, new_status);
        self.display();
        // Not working as intended. It's adding a new line and not deleting the one before.
        // Only option as of now is to generate an entire new table here
        }

    pub fn display(&self) {
        println!("{}", self.table.to_string());
    }
}
