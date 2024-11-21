use serde::{Deserialize, Serialize};
use std::{
    io::{self, Write},
    process::exit,
};
use tabled::{settings::Style, Table, Tabled};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub Title: String,
    pub Description: String,
    pub isComplete: bool,
}

#[derive(Tabled)]
struct NumberedTask {
    id: usize,
    pub Title: String,
    pub Description: String,
    #[tabled(display_with = "bool_to_status")]
    pub isComplete: bool,
}

fn bool_to_status(completed: &bool) -> String {
    if *completed {
        "Complete".to_string()
    } else {
        "Pending".to_string()
    }
}

pub struct TodoList {
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }

    fn get_numberd_task(&self) -> Vec<NumberedTask> {
        self.tasks
            .iter()
            .enumerate()
            .map(|(i, task)| NumberedTask {
                id: i + 1,
                Title: task.Title.clone(),
                Description: task.Description.clone(),
                isComplete: task.isComplete,
            })
            .collect()
    }

    pub fn add_task(&mut self, desc: &str, title: &str) {
        let task = Task {
            Description: desc.to_string(),
            Title: title.to_string(),
            isComplete: false,
        };

        self.tasks.push(task);
        println!("Task Added Successfully");
    }

    pub fn list_task(&self) {
        if self.tasks.len() == 0 {
            println!("No tasks to list");
        } else {
            let numbered_tasks: Vec<NumberedTask> = self.get_numberd_task();

            let mut table = Table::new(&numbered_tasks);
            println!("{}", table.with(Style::modern_rounded()));
        }
    }

    pub fn remove_task(&mut self, id: usize) {
        if self.tasks.len() == 0 {
            println!("No tasks to remove");
        } else {
            println!("Task Removed Successfully");
            self.tasks.remove(id - 1);
        }
    }

    pub fn change_task_status(&mut self, id: usize) {
        self.tasks[id - 1].isComplete = !self.tasks[id - 1].isComplete;
    }
}
