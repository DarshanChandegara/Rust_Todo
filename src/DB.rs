use rusqlite::{Connection, Result};
use crate::lib::Task;

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

pub fn insert_task(task: &Task, group: &Option<String> ,conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (title, description, isComplete, taskGroup) VALUES (?, ?, ?, ?)", (task.Title.as_str(),
        task.Description.as_str(), task.isComplete, group.as_ref().unwrap().clone())
    )?;
    Ok(())
}
