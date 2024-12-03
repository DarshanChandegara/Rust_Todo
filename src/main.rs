mod DB;
mod common;
mod file;
mod lib;
// use file::print_all_files;
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
