// use serde::{Deserialize, Serialize};
use crate::lib;
use std::{
    fs::{self, File},
    io::Write,
};

pub fn save_json(list: &lib::TodoList, filename: &String) {
    let json = serde_json::to_string_pretty(&list.tasks).unwrap();
    let filepath = format!("files/{}.json", filename);
    let mut file = File::create(filepath).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    println!("Files saved successfully");
}

pub fn load_json(file: &String) -> lib::TodoList {
    let content = read_file(format!("files/{}.json", file).as_str());
    let mut todo = lib::TodoList::new();
    todo.tasks = serde_json::from_str(&content).unwrap();
    todo
}

fn read_file(filepath: &str) -> String {
    let content = fs::read_to_string(filepath).unwrap();
    content
}

pub fn print_all_files() -> bool {
    let path = "files/";
    let mut fileFound: bool = false;
    match fs::read_dir(path) {
        Ok(entries) => {
            println!();
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if let Some(file_name_without_ext) = file_name.split('.').next() {
                            fileFound = true;
                            println!("{}", file_name_without_ext);
                        } else {
                            println!("{}", file_name);
                        }
                    } else {
                        eprintln!("Error: Unable to get file name.");
                    }
                } else {
                    eprintln!("Error reading directory entry.");
                }
            }
            println!();
            fileFound
        }
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            fileFound
        }
    }
}
