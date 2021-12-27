use std::env;
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
    pub fn from_args(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let command = args.next().ok_or("Please specify a command!")?;
        let target = args.next().ok_or("Please specify a target!")?;

        let command = match command.as_str() {
            "a" => Command::CreateTodo,
            "add" => Command::CreateTodo,
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
        Command::CreateTodo => {
            create_todo(&config.target)?;
            Ok(())
        }
        _ => Ok(()),
    }
}

fn create_todo(todo: &str) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("todo.txt")?;

    writeln!(file, "{}", todo)?;

    Ok(())
}
