use std::env;
use std::process;

use rust_todo::{Command, Config};

const TODO_TXT_FILENAME: &str = "todo.txt";
const DONE_TXT_FILENAME: &str = "done.txt";

fn main() {
    let command = Command::from_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);

        process::exit(1);
    });

    if let Err(e) = rust_todo::run(command, Config::new(TODO_TXT_FILENAME, DONE_TXT_FILENAME)) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
