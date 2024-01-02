use std::fs::File;
use std::io::prelude::*;
use serde_json;
use std::collections::HashMap;
use crate::task_status::TaskStatus;

pub fn save_tasks(tasks: &HashMap<String, TaskStatus>) {
    let serialized = serde_json::to_string(tasks).unwrap();
    let mut file = File::create("tasks.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
}

pub fn load_tasks() -> HashMap<String, TaskStatus> {
    let mut file = match File::open("tasks.json") {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to open tasks.json: {:?}", e);
            return HashMap::new();
        },
    };
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        println!("Failed to read from tasks.json: {:?}", e);
        return HashMap::new();
    }
    match serde_json::from_str(&contents) {
        Ok(tasks) => tasks,
        Err(e) => {
            println!("Failed to deserialize tasks: {:?}", e);
            HashMap::new()
        }
    }
}
