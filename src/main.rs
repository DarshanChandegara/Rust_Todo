mod common;
mod file;
mod lib;
use std::io::{self, Write};

// use file::print_all_files;
mod test;

fn main() {
    let mut list = lib::TodoList::new();

    let mut fileName: Option<String> = None;

    loop {
        common::clear_terminal();
        println!("\nTo-Do List");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Remove Task");
        println!("4. Change Task status");
        println!("5. load json");
        println!("6. Save");
        println!("7. Exit");

        let mut choice = String::new();
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap();

        match choice {
            1 => {
                common::clear_terminal();
                let mut title = String::new();
                let mut description = String::new();

                print!("Enter task title: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut title).unwrap();

                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut description).unwrap();

                list.add_task(title.trim(), description.trim());
            }
            2 => {
                common::clear_terminal();
                list.list_task();

                let mut input = String::new();
                print!("\nPress C to continue: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                println!("{}", input.trim());
                if input.trim() != "c" {
                    println!("Invalid input!");
                    return ;
                }
            }
            3 => {
                common::clear_terminal();
                list.list_task();
                let mut id = String::new();
                print!("Enter task id: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id).unwrap();
                list.remove_task(id.trim().parse().unwrap());
            }
            4 => {
                common::clear_terminal();
                list.list_task();
                let mut id = String::new();
                print!("Enter task id: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id).unwrap();
                list.change_task_status(id.trim().parse().unwrap());
            }
            5 => {
                common::clear_terminal();
                file::print_all_files();
                print!("Enter File Name : ");
                io::stdout().flush().unwrap();
                let mut file = String::new();
                io::stdin().read_line(&mut file).unwrap();
                file = file.trim().to_string();
                list = file::load_json(&file);
                fileName = Some(file);
            }
            6 => {
                if (list.tasks.len() == 0) {
                    println!("No tasks to save");
                } else {
                    match fileName {
                        Some(name) => {
                            file::save_json(&list, &name);
                        }
                        None => {
                            let mut file = String::new();
                            print!("Enter Name : ");
                            io::stdout().flush().unwrap();
                            io::stdin().read_line(&mut file).unwrap();
                            file = file.trim().to_string();
                            file::save_json(&list, &file);
                        }
                    }
                }
            },
            7 => {
                break ;
            }
            _ => {
                break;
            }
        }
    }
}
