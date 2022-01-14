use chrono;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::prelude::*;

pub enum Command {
    CreateTodo,
    DoTodo,
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
            "do" => Command::DoTodo,
            "ls" => Command::ListTodos,
            "list" => Command::ListTodos,
            _ => Command::Unsupported,
        };

        match command {
            Command::CreateTodo => {
                let target = args.next().ok_or("Please specify a target!")?;
                Ok(Config { command, target })
            }
            Command::DoTodo => {
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
        Command::CreateTodo => create_todo(&config.target).or_else(handle_io_err),
        Command::DoTodo => {
            let parse_result = config.target.parse::<usize>();

            match parse_result {
                Ok(todo_number) => complete_todo(todo_number).or_else(handle_io_err),
                Err(_) => Err(format!("Could not find todo number {}!", config.target)),
            }
        }
        Command::ListTodos => list_todos().or_else(handle_io_err),
        Command::Unsupported => Err(String::from("Unsupported command!")),
    }
}

fn handle_io_err(err: std::io::Error) -> Result<(), String> {
    match err.kind() {
        std::io::ErrorKind::NotFound => Err(String::from(
            "todo.txt file not present. Try creating some todos!",
        )),
        _ => Err(err.to_string()),
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
    let file = OpenOptions::new().read(true).open("todo.txt")?;

    for (index, line) in std::io::BufReader::new(file).lines().enumerate() {
        println!("{} {}", index + 1, line.unwrap());
    }

    Ok(())
}

fn complete_todo(todo_number: usize) -> Result<(), std::io::Error> {
    let todos = fs::read_to_string("todo.txt")?;
    let mut todos_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("todo.txt")?;
    let mut done_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("done.txt")?;

    for (index, line) in todos.lines().enumerate() {
        if (index + 1) == todo_number {
            writeln!(done_file, "x {:?} {}", chrono::offset::Utc::now(), line)?;
        } else {
            writeln!(todos_file, "{}", line)?;
        }
    }

    Ok(())
}
