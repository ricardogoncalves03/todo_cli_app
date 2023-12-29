use crate::task_group::TaskGroup;

use comfy_table::{Table, ContentArrangement, Cell, CellAlignment};

pub struct ToDoTable {
    table: Table,
}

impl ToDoTable {
    pub fn new() -> Self {
        let mut table = Table::new();
        table
            .set_header(vec!["Backlog", "To do", "Done"])
            .set_content_arrangement(ContentArrangement::DynamicFullWidth);
        Self { table }
    }

    pub fn add_to_backlog(&mut self, task: String) {
        self.table.add_row(vec![task, String::new(), String::new()]);
    }

    pub fn add_to_todo(&mut self, task: String) {
        self.table.add_row(vec![String::new(), task, String::new()]);
    }

    pub fn add_to_done(&mut self, task: String) {
        self.table.add_row(vec![String::new(), String::new(), task]);
    }

    pub fn display(&self) {
        println!("{}", self.table.to_string());
    }
}