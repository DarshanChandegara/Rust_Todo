use crate::file::print_all_files;
use crate::lib::{display_status_bar, num_to_status, Task, TodoList};
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

pub fn take_user_input_for_task() -> String {
    let mut input = String::new();
    println!("Task Stages");
    println!("1] To-Do ");
    println!("2] Doing ");
    println!("3] Review ");
    println!("4] Completed ");
    print!("Enter Task Stage Number:- ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<u8>();

    num_to_status(&input.unwrap())
}

pub fn prompt_field_update(id: usize, group: &Option<String>, conn: &Connection) -> Result<(Task)> {
    clear_terminal();
    let mut task = DB::get_task(id, group, conn)?;

    println!("Enter # for filed to not to update Otherwise enter new value");
    let mut title = take_user_input("Enter task title: ");
    let mut description = take_user_input("Enter task description: ");
    let mut status = take_user_input_for_task();

    if title.trim() != "#" {
        task.Title = title.trim().to_string();
    }
    if description.trim() != "#" {
        task.Description = description.trim().to_string();
    }
    if status.trim() != "#" {
        task.stage = status.trim().parse().unwrap();
    }
    Ok(task)
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
            } else {
                break;
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
        println!("5. Search Task");
        println!("6. Show Specific Task Status");
        println!("7. Save");
        println!("8. Exit");

        let choice = take_user_input("Enter your choice: ");
        let choice: u32 = choice.trim().parse().unwrap();

        match choice {
            1 => {
                clear_terminal();
                let mut title = take_user_input("Enter task title: ");
                let mut description = take_user_input("Enter task description: ");
                let mut stage = take_user_input_for_task();

                let id = DB::insert_task(
                    &crate::lib::Task {
                        Title: title.clone().trim().to_string(),
                        Description: description.clone().trim().to_string(),
                        stage: stage.clone(),
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
                clear_terminal();
                println!("1 :- Based On Title");
                println!("2 :- Based On Group");
                let choice = take_user_input("Enter Choice :- ");
                let choice = choice.trim().parse::<u8>().unwrap();

                let mut tasks: Result<(Vec<Task>)> = Ok(Vec::new());

                if choice == 1 {
                    let value = take_user_input("Enter Value :- ");
                    tasks = DB::search_task("Title".to_string(), value.trim().to_string(), conn);
                } else if choice == 2 {
                    let value = take_user_input("Enter Value :- ");
                    tasks =
                        DB::search_task("taskGroup".to_string(), value.trim().to_string(), conn);
                } else {
                    println!("Invalid input!");
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }

                match tasks {
                    Ok(tasks) => {
                        let mut table = Table::new(&tasks);
                        println!("{}", table.with(Style::modern_rounded()));
                        let mut input = take_user_input("\nPress C to continue: ");
                        println!("{}", input.trim());
                        if input.trim() != "c" {
                            println!("Invalid input!");
                            return;
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            6 => {
                clear_terminal();
                if list.tasks.len() == 0 {
                    println!("No tasks to Show");
                    thread::sleep(Duration::from_secs(1));
                } else {
                    list.list_task();
                    let id = take_user_input("Enter id :- ");
                    let id = id.trim().parse::<usize>();

                    let task = DB::get_task(id.unwrap(), &fileName, conn);
                    display_status_bar(task.unwrap().stage);

                    let mut input = take_user_input("\nPress C to continue: ");
                    println!("{}", input.trim());
                    if input.trim() != "c" {
                        println!("Invalid input!");
                        return;
                    }
                }
            }
            7 => {
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
            8 => {
                break;
            }
            _ => {
                break;
            }
        }
    }
}
