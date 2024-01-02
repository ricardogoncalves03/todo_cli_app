use std::fmt;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Backlog,
    ToDo,
    Done,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TaskStatus::Backlog => "Backlog",
                TaskStatus::ToDo => "To Do",
                TaskStatus::Done => "Done",
            }
        )
    }
}
