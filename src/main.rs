mod lib;
use std::io::{self, Write};
mod test;

fn main() {
    let mut list = lib::TodoList::new();
    
    loop {
        lib::clear_terminal();
        println!("\nTo-Do List");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Remove Task");
        println!("4. Change Task status");

        let mut choice = String::new();
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap();

        match choice {
            1 => {
                lib::clear_terminal();
                let mut title = String::new();
                let mut description = String::new();
                
                print!("Enter task title: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut title).unwrap();
                
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut description).unwrap();
                
                list.add_task(title.trim().clone(), description.trim().clone());
            },
            2 => {
                list.list_task();
            },
            3 => {
                lib::clear_terminal();
                let mut id = String::new();
                print!("Enter task id: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id).unwrap();
                list.remove_task(id.trim().parse().unwrap());
            },
            4 => {
                lib::clear_terminal();
                let mut id = String::new();
                print!("Enter task id: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id).unwrap();
                list.change_task_status(id.trim().parse().unwrap());
            },
            _ => {
                break;
            }
        }
    }
}
