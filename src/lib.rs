use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};
use std::{io::{self, stdout, Write}, process::exit};
use serde::{Serialize, Deserialize};
pub fn clear_terminal() {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub Title: String , 
    pub Description: String,
    pub isComplete: bool
}

pub struct TodoList {
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }

    pub fn add_task(&mut self , desc: &str , title: &str) {
        let task = Task {
            Description: desc.to_string(),
            Title: title.to_string(),
            isComplete: false
        };

        self.tasks.push(task);
        println!("Task Added Successfully");

    }

    pub fn list_task(& self) {
        clear_terminal();
        for (i , task) in self.tasks.iter().enumerate() {
            println!("\n\nNumber : {}\nTitle :- {}\nDescripton :- {}\nStatus :- {}" ,i+1,task.Title , task.Description ,if task.isComplete {"Completed"} else {"Pending"});
        }

        let mut input = String::new();
        print!("Press C to continue: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        println!("{}" , input.trim());
        if input.trim() == "c"{
            clear_terminal();
        }else {
            exit(1);
        }
    }

    pub fn remove_task(&mut self ,id: usize) {
        if self.tasks.len() == 0 {
            println!("No tasks to remove");
        } else {
            println!("Task Removed Successfully");
            self.tasks.remove(id-1);
        }
    }

    pub fn change_task_status(&mut self , id: usize) {
        self.tasks[id-1].isComplete = !self.tasks[id-1].isComplete;
    }
}