use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub enum Command {
    CreateTodo,
    ListTodos,
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
        let command = match command.as_str() {
            "a" => Command::CreateTodo,
            "add" => Command::CreateTodo,
            "l" => Command::ListTodos,
            "list" => Command::ListTodos,
            _ => Command::Unsupported,
        };

        match command {
            Command::CreateTodo => {
                let target = args.next().ok_or("Please specify a target!")?;
                Ok(Config { command, target })
            }
            Command::ListTodos => Ok(Config {
                command,
                target: String::from("todo.txt"),
            }),
            Command::Unsupported => Ok(Config {
                command,
                target: String::from(""),
            }),
        }
    }
}

pub fn run(config: Config) -> Result<(), String> {
    match config.command {
        Command::CreateTodo => create_todo(&config.target).or_else(|err| Err(err.to_string())),
        Command::ListTodos => list_todos().or_else(|err| Err(err.to_string())),
        Command::Unsupported => Err(String::from("Unsupported command!")),
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

fn list_todos() -> Result<(), std::io::Error> {
    let file = OpenOptions::new().read(true).open("todo.txt");

    match file {
        Ok(file) => {
            for (index, line) in std::io::BufReader::new(file).lines().enumerate() {
                if let Ok(todo_line) = line {
                    println!("{} {}", index + 1, todo_line);
                }
            }

            Ok(())
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                println!("No todo.txt file yet!");

                Ok(())
            }
            _ => Err(error),
        },
    }
}
