pub mod files;
pub mod todo;
pub mod utils;

pub use crate::utils::*;
use std::env;

use text_io::read;
use todo::action::*;
use todo::*;

pub struct Config {
    pub subcommand: String,
    pub args: Vec<String>,
    pub todofile: String,
    pub is_help: bool,
}

impl Config {
    pub fn new(mut env_args: env::Args, todofile: String) -> Result<Config, &'static str> {
        env_args.next(); // skip program name

        let subcommand = match env_args.next() {
            Some(arg) => arg,
            None => return Err("Missing subcommand"),
        };

        let args: Vec<String> = env_args.collect();
        let is_help = args
            .iter()
            .filter(|s| s.starts_with("-"))
            .map(|s| s.trim_start_matches('-').to_lowercase())
            .filter(|s| s == &"help" || s == &"h")
            .collect::<Vec<_>>()
            .len()
            > 0;

        return Ok(Config {
            subcommand,
            args,
            todofile,
            is_help,
        });
    }

    pub fn get_arg_at<'a>(&'a self, i: usize) -> &'a str {
        return self.args.get(i).map_or("", |s| s);
    }
}

pub fn handle_get_cmd(config: &Config) {
    let filename = config.todofile.as_str();
    let subcommand = config.get_arg_at(0);
    let is_help = config.is_help;

    match subcommand {
        "all" | "-A" => {
            if is_help {
                println!("{}", GET_ALL_HELP);
                return;
            }
            print_all_todos(filename);
        }
        "primary" | "-P" => {
            if is_help {
                println!("{}", GET_PRIMARY_HELP);
                return;
            }
            print_primary_todo(filename);
        }
        "title" | "-T" => {
            if is_help {
                println!("{}", GET_TITLE_HELP);
                return;
            }
            let title = config.get_arg_at(1);
            if title.is_empty() {
                eprintln!("{}", MISSING_TODO_TITLE_ERR);
                std::process::exit(1);
            }

            print_todo_by_title(title, filename);
        }
        _ => {
            eprintln!("Unknown command \"{}\"", subcommand);
            eprintln!("{}\n", GET_SUBCMDS);
        }
    }
}

pub fn handle_add_cmd(config: &Config) {
    if config.is_help {
        println!("{}", ADD_SUBCMDS);
        return;
    }

    loop {
        println!("Name of the todo?");
        let name: String = read!("{}\n");

        println!("Priority of the todo? (number)");
        let priority: u32 = read!("{}\n");

        let title = format!("[{}]", name);
        let mut todo: Todo = Todo::new(title, priority);

        print!("Adding details to the todo. ");
        println!("Exit by typing \"q\" or \"quit\"");

        loop {
            let item: String = read!("{}\n");
            match item.to_lowercase().as_str() {
                "q" | "quit" => break,
                _ => todo.add_item(item),
            };
        }

        println!("Adding the following todo:\n\n{}", todo.to_string());
        println!("Continue?");
        println!("Continue [ok/1], Recreate [re/2], Quit [q/3]");
        let is_ok: String = read!("{}\n");

        match is_ok.to_lowercase().as_str() {
            "ok" | "1" => {
                add_todo(&config.todofile, todo);
                break;
            }
            "re" | "2" => {
                println!("Recreating the todo\n");
                continue;
            }
            _ => break,
        };
    }
}

pub fn handle_rm_cmd(config: &Config) {
    let filename = config.todofile.as_str();
    let subcommand = config.get_arg_at(0);
    let is_help = config.is_help;

    match subcommand {
        "all" | "-A" => {
            if is_help {
                println!("{}", RM_ALL_HELP);
                return;
            }

            remove_all_todos(filename);
        }
        "primary" | "-P" => {
            if is_help {
                println!("{}", RM_PRIMARY_HELP);
                return;
            }

            remove_primary_todo(filename);
        }
        "title" | "-T" => {
            if is_help {
                println!("{}", RM_TITLE_HELP);
                return;
            }
            let title = config.get_arg_at(1);
            if title.is_empty() {
                eprintln!("{}", MISSING_TODO_TITLE_ERR);
                std::process::exit(1);
            }

            remove_todo_by_title(title, filename);
        }
        _ => {
            eprintln!("Unknown command \"{}\"", subcommand);
            eprintln!("{}\n", RM_SUBCMDS);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use files::{delete_file, write_file};
    const FILENAME: &str = "__test.tudu";

    #[test]
    fn test_get_todos_from_file() {
        assert!(
            write_file(FILENAME, "[A]!!!\na1\na2\na3\n[B]\nb1\nb2").is_ok(),
            "should create todo file"
        );
        let todos = get_todos(FILENAME);
        assert!(todos.len() > 0, "todos should not be empty");
        assert!(delete_file(FILENAME).is_ok(), "should delete todo file");
    }

    #[test]
    fn test_parse_title() {
        let (title, priority) = parse_title("[This a test]!!!!!");
        assert_eq!(title, "[This a test]".to_string());
        assert_eq!(priority, 5);
    }

    #[test]
    fn test_get_todos_by_title() {
        let title = "[test_todo]";
        let todos = vec![Todo::new(title.to_string(), 3)];
        let todos = include_todos_by_title(&todos, &title);
        assert!(todos.len() > 0, "should not be empty");
        assert_eq!(todos.first().unwrap().title, title);
    }

    #[test]
    fn test_exclude_todos_by_title() {
        let title = "[test_todo]";
        let todos = vec![Todo::new(title.to_string(), 3)];
        let todos = exclude_todos_by_title(&todos, &title);
        assert_eq!(todos.len(), 0, "should empty by excluding the [test_todo]");
    }
}
