use crate::file::print_all_files;
use crate::lib::{self, Task, TodoList};
use crate::{file, DB};
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use rusqlite::{Connection, Result};
use std::io::{self, stdout, Write};
use std::process::exit;
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

pub fn prompt_field_update(id: usize, group: &Option<String>, conn: &Connection) -> Result<(Task)> {
    clear_terminal();
    let mut task = DB::get_task(id, group, conn)?;

    println!("Enter # for filed to not to update Otherwise enter new value");
    let mut title = take_user_input("Enter task title: ");
    let mut description = take_user_input("Enter task description: ");
    let mut status = take_user_input("Enter task status: (false / true)");

    if title.trim() != "#" {
        task.Title = title.trim().to_string();
    }
    if description.trim() != "#" {
        task.Description = description.trim().to_string();
    }
    if status.trim() != "#" {
        task.isComplete = status.trim().parse().unwrap();
    }
    Ok((task))
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

    loop {
        clear_terminal();
        println!("\nTo-Do List");
        println!("1. Load tasks from file");
        println!("2. Load tasks from DB");
        println!("3. Create New Task group");
        println!("4. Exit");

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
            3 => {
                let mut file = take_user_input("Enter File Name : ");
                file = file.trim().to_string();
                fileName = Some(file);
                break;
            }
            4 => {
                exit(0);
            }
            _ => {
                return;
            }
        }

        if let Some(found) = fileFound {
            if found == false {
                println!("No Files to Load");
                let mut input = take_user_input("\nPress C to continue: ");
                println!("{}", input.trim());
                if input.trim() != "c" {
                    println!("Invalid input!");
                    return;
                }
            }
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
        println!("4. Update Task");
        println!("5. Save");
        println!("6. Exit");

        let choice = take_user_input("Enter your choice: ");
        let choice: u32 = choice.trim().parse().unwrap();

        match choice {
            1 => {
                clear_terminal();
                let mut title = take_user_input("Enter task title: ");
                let mut description = take_user_input("Enter task description: ");

                let id = DB::insert_task(
                    &crate::lib::Task {
                        Title: title.clone().trim().to_string(),
                        Description: description.clone().trim().to_string(),
                        isComplete: false,
                        id: 0,
                    },
                    &fileName,
                    conn,
                );
                list.add_task(id.unwrap(), title.trim(), description.trim());
                thread::sleep(Duration::from_secs(1));
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
                list.list_task();
                let id = take_user_input("Enter task id: ");
                match DB::delete_task(id.trim().parse().unwrap(), &fileName, conn) {
                    Ok(()) => {
                        list.remove_task(id.trim().parse().unwrap());
                        thread::sleep(Duration::from_secs(1));
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            4 => {
                clear_terminal();
                list.list_task();
                let mut id = take_user_input("Enter task id: ");
                id = id.trim().parse().unwrap();
                match prompt_field_update(id.trim().parse().unwrap(), &fileName, conn) {
                    Ok(task) => match DB::update_task(&task, &fileName, conn) {
                        Ok(()) => {
                            list.update(id.trim().parse().unwrap(), task);
                            thread::sleep(Duration::from_secs(1));
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    },
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
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
