use std::env;
use std::process;

use rust_todo::Command;

fn main() {
    let command = Command::from_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);

        process::exit(1);
    });

    if let Err(e) = rust_todo::run(command) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
