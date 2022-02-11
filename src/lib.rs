use chrono;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::prelude::*;
use std::io::{BufReader, ErrorKind};

pub enum Command {
    CreateTodo(String),
    DoTodo(usize),
    ListTodos(Option<String>),
    ListAllTodos,
    Unsupported,
}

const TODO_TXT_FILENAME: &str = "todo.txt";
const DONE_TXT_FILENAME: &str = "done.txt";

impl Command {
    pub fn from_args(mut args: env::Args) -> Result<Command, String> {
        args.next();

        let command = args.next().ok_or("Please specify a command!")?;

        return match command.as_str() {
            "a" | "add" => {
                let todo = parse_remaining_args(args).ok_or("Please specify text for a todo!")?;
                Ok(Command::CreateTodo(todo))
            }
            "do" => {
                let target = args.next().ok_or("Please specify a todo number!")?;
                let parse_result = target.parse::<usize>();
                return match parse_result {
                    Ok(todo_number) => Ok(Command::DoTodo(todo_number)),
                    Err(_) => Err(format!("Could not find todo number {}!", &target)),
                };
            }
            "ls" | "list" => Ok(Command::ListTodos(parse_remaining_args(args))),
            "la" | "listall" => Ok(Command::ListAllTodos),
            _ => Ok(Command::Unsupported),
        };
    }
}

pub fn run(command: Command) -> Result<(), String> {
    match command {
        Command::CreateTodo(todo) => create_todo(&todo)
            .or_else(handle_io_err)
            .and(list_todos().or_else(handle_io_err)),
        Command::DoTodo(todo_number) => complete_todo(todo_number)
            .or_else(handle_io_err)
            .and(list_todos().or_else(handle_io_err)),
        Command::ListTodos(search) => match search {
            Some(term) => search_todos(&term).or_else(handle_io_err),
            None => list_todos().or_else(handle_io_err),
        },
        Command::ListAllTodos => match list_todos().or_else(handle_io_err) {
            Ok(_) => list_done_todos().or_else(handle_io_err),
            Err(err) => Err(err),
        },
        Command::Unsupported => Err(String::from("Unsupported command!")),
    }
}

fn parse_remaining_args(args: env::Args) -> Option<String> {
    let todo = args.reduce(|mut accum, word| {
        let next_word = &format!(" {}", word);
        accum.push_str(next_word);
        accum
    });

    match todo {
        Some(value) => Some(String::from(value.trim())),
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

fn search_todos(search: &str) -> Result<(), std::io::Error> {
    let file = OpenOptions::new().read(true).open(TODO_TXT_FILENAME);
    let mut number_of_todos = 0;
    let mut shown_todos = 0;

    match file {
        Ok(handle) => {
            for (index, line) in BufReader::new(handle).lines().enumerate() {
                number_of_todos = index + 1;
                let todo: &str = &line.unwrap();

                if todo.contains(search) {
                    println!("{} {}", index + 1, todo);
                    shown_todos = shown_todos + 1;
                }
            }

            println!();
            println!("Showing {} of {} todos.", shown_todos, number_of_todos);

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

fn list_todos() -> Result<(), std::io::Error> {
    let file = OpenOptions::new().read(true).open(TODO_TXT_FILENAME);
    let mut number_of_todos = 0;

    match file {
        Ok(handle) => {
            for (index, line) in BufReader::new(handle).lines().enumerate() {
                number_of_todos = index + 1;
                println!("{} {}", index + 1, line.unwrap());
            }

            println!();
            println!("Showing {} of {} todos.", number_of_todos, number_of_todos);

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
