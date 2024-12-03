use crate::common::group;
use crate::lib::{NumberedTask, Task, TodoList};
use rusqlite::{Connection, Result};

pub fn database_init() -> Result<Connection> {
    let conn = Connection::open("./tasks.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT,
            description TEXT,
            isComplete BOOL ,
            taskGroup TEXT 
        )",
        (),
    )?;
    Ok(conn)
}

pub fn insert_task(task: &Task, group: &Option<String>, conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (title, description, isComplete, taskGroup) VALUES (?, ?, ?, ?)",
        (
            task.Title.as_str(),
            task.Description.as_str(),
            task.isComplete,
            group.as_ref().unwrap().clone(),
        ),
    )?;
    Ok(())
}

pub fn get_task(group: &Option<String>, conn: &Connection) -> Result<(Vec<NumberedTask>)> {
    let query = format!(
        "SELECT id, title, description, isComplete FROM tasks WHERE taskGroup = \"{}\"",
        group.as_ref().unwrap()
    );
    let mut stmt = conn.prepare(&query.as_str())?;

    let tasks_iter = stmt.query_map([], |row| {
        let id = row.get::<_, usize>(0)?;
        let Title = row.get::<_, String>(1)?;
        let Description = row.get::<_, String>(2)?;
        let isComplete = row.get::<_, bool>(3)?;

        println!(
            "ID: {}, Title: {}, Description: {}, isComplete: {}",
            id, Title, Description, isComplete
        );
        Ok(NumberedTask {
            id,
            Title,
            Description,
            isComplete,
        })
    })?;
    let mut tasks: Vec<NumberedTask> = Vec::new();
    for task in tasks_iter {
        tasks.push(task.unwrap());
    }
    println!("{:#?}", tasks);
    Ok(tasks)
}

pub fn show_groups(conn: &Connection) -> Result<(Vec<group>)> {
    let query = format!("SELECT DISTINCT taskGroup FROM tasks");
    let mut stmt = conn.prepare(&query.as_str())?;

    let tasks_iter = stmt.query_map([], |row| {
        let name = row.get::<_, String>(0)?;
        Ok(name)
    })?;
    let mut groups: Vec<group> = Vec::new();
    for (id, group) in tasks_iter.into_iter().enumerate() {
        groups.push(group {
            id: id + 1,
            name: group.unwrap().clone(),
        });
    }
    Ok((groups))
}

pub fn load_tasks(group: &Option<String>, conn: &Connection) -> Result<TodoList> {
    let mut todo = TodoList::new();

    let number_tasks = get_task(group, conn).unwrap();
    let tasks = number_tasks
        .iter()
        .map(|task| Task {
            Title: task.Title.clone().trim().to_string(),
            Description: task.Description.clone().trim().to_string(),
            isComplete: task.isComplete,
        })
        .collect();

    todo.tasks = tasks;
    Ok((todo))
}
