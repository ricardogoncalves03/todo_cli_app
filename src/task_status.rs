use std::fmt;

use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display_format_is_correct() {
        assert_eq!(format!("{}", TaskStatus::Backlog), "Backlog");
        assert_eq!(format!("{}", TaskStatus::ToDo), "To Do");
        assert_eq!(format!("{}", TaskStatus::Done), "Done");
    }

    #[test]
    fn serde_serialization_and_deserialization() {
        let variants = [TaskStatus::Backlog, TaskStatus::ToDo, TaskStatus::Done];

        for variant in &variants {
            let serialized = serde_json::to_string(variant).unwrap();
            let deserialized: TaskStatus = serde_json::from_str(&serialized).unwrap();
            assert_eq!(*variant, deserialized);
        }
    }
}
