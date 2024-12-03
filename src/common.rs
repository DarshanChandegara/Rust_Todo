use crate::file::print_all_files;
use crate::lib::TodoList;
use crate::{file, DB};
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use rusqlite::Connection;
use std::io::{self, stdout, Write};
use std::thread;
use std::time::Duration;
use tabled::{settings::Style, Table, Tabled};

#[derive(Tabled)]
pub struct group {
    pub id: usize,
    pub(crate) name: String,
}

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

pub fn print_all_groups(conn: &Connection) -> bool {
    let grps = DB::show_groups(conn).unwrap();

    if grps.len() == 0 {
        return false;
    }
    let mut table = Table::new(&grps);
    println!("{}", table.with(Style::modern_rounded()));
    return true;
}

pub fn start(conn: &Connection) {
    let mut list = TodoList::new();

    let mut fileName: Option<String> = None;

    clear_terminal();
    println!("\nTo-Do List");
    println!("1. Load tasks from file");
    println!("2. Load tasks from DB");
    println!("3. Create New Task group");

    let choice = take_user_input("Enter your choice: ");
    let choice: u32 = choice.trim().parse().unwrap();
    let mut fileFound: Option<bool> = None;
    match choice {
        1 => {
            clear_terminal();
            fileFound = Some(print_all_files());

            if fileFound == Some(true) {
                let file = take_user_input("Enter File Name : ");
                let file = file.trim().to_string();
                list = file::load_json(&file);
                fileName = Some(file);
            }
        }
        2 => {
            clear_terminal();
            fileFound = Some(print_all_groups(conn));

            if fileFound == Some(true) {
                let file = take_user_input("Enter File Name : ");
                let file = file.trim().to_string();
                list = DB::load_tasks(&Some(file.clone()), &conn).unwrap();
                fileName = Some(file);
            }
        }
        3 => {}
        _ => {
            return;
        }
    }

    match fileFound {
        Some(found) => {
            if found == false {
                println!("No Files to Load");
                let mut input = take_user_input("\nPress C to continue: ");
                println!("{}", input.trim());
                if input.trim() != "c" {
                    println!("Invalid input!");
                    return;
                }

                let mut file = take_user_input("Enter File Name : ");
                file = file.trim().to_string();
                fileName = Some(file);
            }
        }
        None => {
            let mut file = take_user_input("Enter File Name : ");
            file = file.trim().to_string();
            fileName = Some(file);
        }
    }
    run(&mut list, fileName, &conn);
}

fn run(list: &mut TodoList, fileName: Option<String>, conn: &Connection) {
    loop {
        clear_terminal();
        println!("\nTo-Do List");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Remove Task");
        println!("4. Change Task status");
        println!("5. Save");
        println!("6. Exit");

        let choice = take_user_input("Enter your choice: ");
        let choice: u32 = choice.trim().parse().unwrap();

        match choice {
            1 => {
                clear_terminal();
                let mut title = take_user_input("Enter task title: ");
                let mut description = take_user_input("Enter task description: ");

                let _ = DB::insert_task(
                    &crate::lib::Task {
                        Title: title.clone().trim().to_string(),
                        Description: description.clone().trim().to_string(),
                        isComplete: false,
                    },
                    &fileName,
                    conn,
                );
                list.add_task(title.trim(), description.trim());
            }
            2 => {
                clear_terminal();
                list.list_task();

                let mut input = take_user_input("\nPress C to continue: ");
                println!("{}", input.trim());
                if input.trim() != "c" {
                    println!("Invalid input!");
                    return;
                }
            }
            3 => {
                clear_terminal();
                let numbered_tasks = DB::get_task(&fileName, conn);
                list.list_task();
                let id = take_user_input("Enter task id: ");
                list.remove_task(id.trim().parse().unwrap());
            }
            4 => {
                clear_terminal();
                let numbered_tasks = DB::get_task(&fileName, conn);
                list.list_task();
                let id = take_user_input("Enter task id: ");
                list.change_task_status(id.trim().parse().unwrap());
            }
            5 => {
                if list.tasks.len() == 0 {
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
