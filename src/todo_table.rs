use crate::task_status::TaskStatus;
use comfy_table::{ContentArrangement, Table};
use std::collections::HashMap;

pub struct ToDoTable {
    table: Table,
    pub tasks: HashMap<String, TaskStatus>,
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

            // Rebuild the table to reflect the updated task status
            self.rebuild_table();
        } else {
            println!("Task not found.");
        }
    }

    pub fn delete_task(&mut self, task: &str) {
        if self.tasks.remove(task).is_some() {
            // If the task was successfully removed, rebuild the table
            self.rebuild_table();
        } else {
            println!("Task not found.");
        }
    }

    pub fn rebuild_table(&mut self) {
        let mut new_table = Self::create_table_with_headers();
        for (task_name, &status) in &self.tasks {
            let row = match status {
                TaskStatus::Backlog => vec![task_name.to_string(), "".to_string(), "".to_string()],
                TaskStatus::ToDo => vec!["".to_string(), task_name.to_string(), "".to_string()],
                TaskStatus::Done => vec!["".to_string(), "".to_string(), task_name.to_string()],
            };
            new_table.add_row(row);
        }
        self.table = new_table;
    }

    pub fn display(&self) {
        println!("{}", self.table.to_string());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut table = ToDoTable::new();
        table.add_task("Test Task 1".to_string(), TaskStatus::Backlog);

        assert_eq!(table.tasks.contains_key("Test Task 1"), true);
        assert_eq!(
            *table.tasks.get("Test Task 1").unwrap(),
            TaskStatus::Backlog
        );
    }

    #[test]
    fn test_update_task_status() {
        let mut table = ToDoTable::new();
        table.add_task("Test Task 2".to_string(), TaskStatus::Backlog);
        table.update_task_status("Test Task 2", TaskStatus::ToDo);

        assert_eq!(*table.tasks.get("Test Task 2").unwrap(), TaskStatus::ToDo);
    }

    #[test]
    fn test_delete_task() {
        let mut table = ToDoTable::new();
        table.add_task("Test Task 3".to_string(), TaskStatus::Backlog);
        table.delete_task("Test Task 3");

        assert_eq!(table.tasks.contains_key("Test Task 3"), false);
    }

    #[test]
    fn test_rebuild_table() {
        let mut table = ToDoTable::new();
        table.add_task("Test Task 4".to_string(), TaskStatus::Backlog);
        table.add_task("Test Task 5".to_string(), TaskStatus::ToDo);
        table.rebuild_table();
    }
}
