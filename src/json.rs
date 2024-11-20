use serde::{Deserialize, Serialize};
use std::fs::{self , File};
use crate::lib;

pub fn save_json(list: &lib::TodoList) {
    let json = serde_json::to_string_pretty(&list.tasks).unwrap();
    let mut file = File::create("todo.json").unwrap();
    fs::write("todo.json" , json).unwrap();
}

pub fn load_json() -> lib::TodoList {
    let content = read_file("todo.json");
    let mut todo = lib::TodoList::new();
    todo.tasks = serde_json::from_str(&content).unwrap();
    todo
}

fn read_file(filepath :&str) -> String {
    let content = fs::read_to_string(filepath).unwrap();
    content
}