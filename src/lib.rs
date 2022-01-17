use chrono;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::prelude::*;
use std::io::{BufReader, ErrorKind};

pub enum Command {
    CreateTodo,
    DoTodo,
    ListTodos,
    ListAllTodos,
    Unsupported,
}

pub struct Config {
    command: Command,
    target: String,
}

const TODO_TXT_FILENAME: &str = "todo.txt";
const DONE_TXT_FILENAME: &str = "done.txt";

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
            "listall" => Command::ListAllTodos,
            _ => Command::Unsupported,
        };

        match command {
            Command::CreateTodo => Ok(Config {
                command,
                target: parse_remaining_args(args).ok_or("Please specify text for a todo!")?,
            }),
            Command::DoTodo => {
                let target = args.next().ok_or("Please specify a target!")?;
                Ok(Config { command, target })
            }
            Command::ListTodos => Ok(Config {
                command,
                target: String::from(TODO_TXT_FILENAME),
            }),
            Command::ListAllTodos => Ok(Config {
                command,
                target: String::from(TODO_TXT_FILENAME),
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
        Command::ListAllTodos => match list_todos().or_else(handle_io_err) {
            Ok(_) => list_done_todos().or_else(handle_io_err),
            Err(err) => Err(err),
        },
        Command::Unsupported => Err(String::from("Unsupported command!")),
    }
}

fn parse_remaining_args(args: env::Args) -> Option<String> {
    let target = args.reduce(|mut accum, word| {
        accum.push_str(&format!("{} ", &word));
        accum
    });

    match target {
        Some(value) => Some(String::from(value.trim_end())),
        None => None,
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
        .open(TODO_TXT_FILENAME)?;

    writeln!(file, "{}", todo)?;

    Ok(())
}

fn list_todos() -> Result<(), std::io::Error> {
    let file = OpenOptions::new().read(true).open(TODO_TXT_FILENAME);

    match file {
        Ok(handle) => {
            for (index, line) in BufReader::new(handle).lines().enumerate() {
                println!("{} {}", index + 1, line.unwrap());
            }

            Ok(())
        }
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                Ok(()) // None to list
            } else {
                Err(err)
            }
        }
    }
}

fn list_done_todos() -> Result<(), std::io::Error> {
    let file = OpenOptions::new().read(true).open(DONE_TXT_FILENAME);

    match file {
        Ok(handle) => {
            for line in BufReader::new(handle).lines() {
                println!("{} {}", 0, line.unwrap());
            }

            Ok(())
        }
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                Ok(()) // None to list
            } else {
                Err(err)
            }
        }
    }
}

fn complete_todo(todo_number: usize) -> Result<(), std::io::Error> {
    let todos = fs::read_to_string(TODO_TXT_FILENAME)?;
    let mut todos_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(TODO_TXT_FILENAME)?;
    let mut done_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(DONE_TXT_FILENAME)?;

    for (index, line) in todos.lines().enumerate() {
        if (index + 1) == todo_number {
            writeln!(done_file, "x {:?} {}", chrono::offset::Utc::now(), line)?;
        } else {
            writeln!(todos_file, "{}", line)?;
        }
    }

    Ok(())
}
