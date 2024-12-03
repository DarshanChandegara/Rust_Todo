mod DB;
mod common;
mod file;
mod lib;
mod test;

fn main() {
    let result = DB::database_init();
    match result {
        Ok(conn) => {
            common::start(&conn);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
