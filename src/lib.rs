use serde::{Deserialize, Serialize};
use tabled::{settings::Style, Table, Tabled};

#[derive(Serialize, Deserialize, Debug , Tabled)]
pub struct Task {
    pub id: usize,
    pub Title: String,
    pub Description: String,
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

    pub fn add_task(&mut self, id: usize, title: &str, desc: &str) {
        let task = Task {
            id : id ,
            Title: title.to_string(),
            Description: desc.to_string(),
            isComplete: false,
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
            println!("Task Removed Successfully");
            self.tasks.remove(id - 1);
        }
    }

    pub fn change_task_status(&mut self, id: usize) {
        self.tasks[id - 1].isComplete = !self.tasks[id - 1].isComplete;
    }
}
