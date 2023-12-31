use crate::task_status::TaskStatus;
use comfy_table::{ContentArrangement, Table};
use std::collections::HashMap;

pub struct ToDoTable {
    table: Table,
    tasks: HashMap<String, TaskStatus>,
}

impl ToDoTable {
    pub fn new() -> Self {
        Self {
            table: Self::create_table_with_headers(),
            tasks: HashMap::new(),
        }
    }

    fn create_table_with_headers() -> Table {
        let mut table = Table::new();

        table
            .set_header(vec![
                TaskStatus::Backlog.to_string(),
                TaskStatus::ToDo.to_string(),
                TaskStatus::Done.to_string(),
            ])
            .set_content_arrangement(ContentArrangement::DynamicFullWidth);
        table
    }

    pub fn add_task(&mut self, task: String, status: TaskStatus) {
        self.tasks.insert(task.clone(), status);

        match status {
            TaskStatus::Backlog => self
                .table
                .add_row(vec![task, "".to_string(), "".to_string()]),
            TaskStatus::ToDo => self
                .table
                .add_row(vec!["".to_string(), task, "".to_string()]),
            TaskStatus::Done => self
                .table
                .add_row(vec!["".to_string(), "".to_string(), task]),
        };
    }

    pub fn update_task_status(&mut self, task: &str, new_status: TaskStatus) {
        if let Some(status) = self.tasks.get_mut(task) {
            *status = new_status;

            //Create a new table
            let mut new_table = Self::create_table_with_headers();

            for (task_name, &status) in &self.tasks {
                let row = match status {
                    TaskStatus::Backlog => {
                        vec![task_name.to_string(), "".to_string(), "".to_string()]
                    }
                    TaskStatus::ToDo => vec!["".to_string(), task_name.to_string(), "".to_string()],
                    TaskStatus::Done => vec!["".to_string(), "".to_string(), task_name.to_string()],
                };
                new_table.add_row(row);
            }
            self.table = new_table;
        }
    }

    pub fn display(&self) {
        println!("{}", self.table.to_string());
    }
}
