use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub task_name: String,
    pub task_description: String,
    pub task_complete: String,
}

pub fn load_tasks() -> Vec<Task> {
    let mut tasks = Vec::new();

    let mut file = File::open("tasks.csv").unwrap_or_else(|_| File::create("tasks.csv").unwrap());
    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => {
            let mut reader = Reader::from_reader(contents.as_bytes());
            for result in reader.deserialize() {
                let task: Task = result.unwrap();
                tasks.push(task);
            }
        }
        Err(_) => (),
    };
    tasks
}

pub fn save_tasks(tasks: Vec<Task>) {
    let file = File::create("tasks.csv")
        .ok()
        .expect("Failed to create tasks.csv");
    let mut writer = Writer::from_writer(file);

    for task in tasks {
        writer
            .serialize(task)
            .ok()
            .expect("Failed to serialize task");
    }
}
