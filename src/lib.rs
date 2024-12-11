use ansi_term::Colour;
use serde::{Deserialize, Serialize};
use tabled::{settings::Style, Table, Tabled};
#[derive(Serialize, Deserialize, Debug, Tabled)]
pub struct Task {
    pub id: usize,
    pub Title: String,
    pub Description: String,
    pub stage: String,
}

pub fn num_to_status(progress: &u8) -> String {
    if *progress == 1 {
        "To-Do".to_string()
    } else if *progress == 2 {
        "Doing".to_string()
    } else if *progress == 3 {
        "Review".to_string()
    } else if *progress == 4 {
        "Done".to_string()
    } else {
        "Invalid state".to_string()
    }
}

pub fn display_status_bar(status: String) {
    let stages = ["To-Do", "Doing", "Review", "Done"];
    let mut current_index;

    if status == "To-Do".to_string() {
        current_index = 0;
    } else if status == "Doing".to_string() {
        current_index = 1;
    } else if status == "Review".to_string() {
        current_index = 2;
    } else if status == "Done".to_string() {
        current_index = 3;
    } else {
        current_index = 4;
    }

    if current_index > 3 {
        println!("Invalid Status");
        return;
    }

    for (i, stage) in stages.iter().enumerate() {
        if i < current_index {
            print!(
                "{} {} ",
                Colour::Green.bold().paint("●"),
                Colour::Green.paint(*stage)
            );
        } else if i == current_index {
            if i == 3 {
                print!(
                    "{} {} ",
                    Colour::Green.bold().paint("●"),
                    Colour::Green.paint(*stage)
                );
            } else {
                print!(
                    "{} {} ",
                    Colour::Yellow.bold().paint("◉"),
                    Colour::Yellow.paint(*stage)
                );
            }
        } else {
            print!(
                "{} {} ",
                Colour::Red.paint("○"),
                Colour::Red.dimmed().paint(*stage)
            );
        }

        if i != stages.len() - 1 {
            print!("{} ", Colour::White.paint("─────"));
        }
    }

    println!();
}

pub struct TodoList {
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, id: usize, title: &str, desc: &str) {
        let task = Task {
            id: id,
            Title: title.to_string(),
            Description: desc.to_string(),
            stage: "To-Do".to_string(),
        };

        self.tasks.push(task);
        println!("Task Added Successfully");
    }

    pub fn list_task(&self) {
        if self.tasks.len() == 0 {
            println!("No tasks to list");
        } else {
            let mut table = Table::new(&self.tasks);
            println!("{}", table.with(Style::modern_rounded()));
        }
    }

    pub fn remove_task(&mut self, id: usize) {
        if self.tasks.len() == 0 {
            println!("No tasks to remove");
        } else {
            self.tasks.retain(|task| task.id != id);
            println!("Task Removed Successfully");
        }
    }

    pub fn update(&mut self, id: usize, task: Task) {
        if self.tasks.len() == 0 {
            println!("No tasks to update");
        } else {
            self.tasks.retain(|task| task.id != id);
            self.tasks.push(task);
            println!("Task Updated Successfully");
        }
    }
}
