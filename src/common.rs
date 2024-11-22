use crate::file::{self, print_all_files};
use crate::lib::TodoList;
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, stdout, Write};
use std::thread;
use std::time::Duration;

pub fn take_user_input(msg: &str) -> String {
    let mut input = String::new();
    print!("{msg}");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn clear_terminal() {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();
}

pub fn start() {
    let mut list = TodoList::new();

    let mut fileName: Option<String> = None;

    clear_terminal();
    println!("\nTo-Do List");
    println!("1. Load Data from file");
    println!("2. Create New Task");

    let choice=  take_user_input("Enter your choice: ");
    let choice :u32 =choice.trim().parse().unwrap();

    match choice {
        1 => {
            clear_terminal();
            let fileFound = file::print_all_files();

            if fileFound {
                let file = take_user_input("Enter File Name : ");
                let file  = file.trim().to_string();
                list = file::load_json(&file);
                fileName = Some(file);
            } else {
                println!("No Files to Load");

                let mut input = take_user_input("\nPress C to continue: ");
                println!("{}", input.trim());
                if input.trim() != "c" {
                    println!("Invalid input!");
                    return;
                }
            }
        }
        2 => {

            let mut file = take_user_input("Enter File Name : ");
            file = file.trim().to_string();
            fileName = Some(file);
            
        }
        _ => { return ;}
    }
    run(&mut list, fileName);
}

fn run(list: &mut TodoList, fileName: Option<String>) {
    loop {
        clear_terminal();
        println!("\nTo-Do List");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Remove Task");
        println!("4. Change Task status");
        println!("5. Save");
        println!("6. Exit");

        let choice=  take_user_input("Enter your choice: ");
        let choice :u32 =choice.trim().parse().unwrap();

        match choice {
            1 => {
                clear_terminal();
                let mut title = take_user_input("Enter task title: ");
                let mut description = take_user_input("Enter task description: ");

                list.add_task(title.trim(), description.trim());
            }
            2 => {
                clear_terminal();
                list.list_task();

                let mut input = take_user_input( "\nPress C to continue: ");
                println!("{}", input.trim());
                if input.trim() != "c" {
                    println!("Invalid input!");
                    return;
                }
            }
            3 => {
                clear_terminal();
                list.list_task();
                let id = take_user_input("Enter task id: ");
                list.remove_task(id.trim().parse().unwrap());
            }
            4 => {
                clear_terminal();
                list.list_task();
                let id = take_user_input("Enter task id: ");
                list.change_task_status(id.trim().parse().unwrap());
            }
            5 => {
                if (list.tasks.len() == 0) {
                    println!("No tasks to save");
                    thread::sleep(Duration::from_secs(1));
                } else {
                    match fileName.as_ref() {
                        Some(name) => {
                            file::save_json(&list, &name);
                            thread::sleep(Duration::from_secs(1));
                        }
                        None => {
                            println!("No file name provided");
                        }
                    }
                }
            }
            6 => {
                break;
            }
            _ => {
                break;
            }
        }
    }
}