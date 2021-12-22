use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub enum Command {
    CreateTodo,
    Unsupported,
}

pub struct Config {
    command: Command,
    target: String,
}

impl Config {
    pub fn from_args(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let command = args[1].clone();
        let target = args[2].clone();

        let command = match command.as_str() {
            "c" => Command::CreateTodo,
            _ => Command::Unsupported,
        };

        match command {
            Command::Unsupported => Err("Unsupported command!"),
            _ => Ok(Config { command, target }),
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.command {
        Command::CreateTodo => create_todo(&config.target),
        _ => (),
    }

    Ok(())
}

fn create_todo(todo: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("todo.txt")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", todo) {
        eprintln!("Could not write: '{}' to file: {}", todo, "todo.txt");
        eprintln!("{}", e);
    }
}
