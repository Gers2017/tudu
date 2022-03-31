pub mod utils;
pub mod todo;
pub mod files;
pub use crate::utils::*;

use text_io::read;
use todo::*;
use todo::action::*;

pub struct Config {
    pub subcommand: String,
    pub args: Vec<String>,
    pub todofile: String,
}

use std::process;
impl Config {
    pub fn new(args: &[String], todofile: String) -> Config {
        if args.len() < 2 {
            eprintln!("{}", AVAILABLE_CMDS);
            process::exit(1);
        }
        
        let subcommand = args[1].clone();    
        Config{subcommand, args: args.to_vec(), todofile }
    }
}

pub fn handle_get_cmd(config: Config){
    if config.args.len() < 2 {
        eprintln!("{}\n", GET_SUBCMDS);
        return;
    }

    let subcommand = config.args.get(2).unwrap();

    let filename = config.todofile;
    match subcommand.as_str() {
        "all" | "-A" => print_all_todos(&filename),
        "primary" | "-P" => print_primary_todo(&filename),
        "title" | "-T" => print_todo_by_title(&config.args, &filename),
        _ => {
            eprintln!("Unknown command \"{}\"", subcommand);
            eprintln!("{}\n", GET_SUBCMDS);
        },
    }
}

pub fn handle_add_cmd(config: Config){
    loop {
        println!("Name of the todo?");
        let name: String = read!("{}\n");
        
        println!("Priority of the todo?");
        let priority: u32 = read!("{}\n");
    
        let title = format!("[{}]", name);
        let mut todo: Todo = Todo::new(title, priority);
        
        print!("Adding details to the todo. ");
        println!("Exit by typing \"exit\" or \"quit\"");
    
        loop {
            let item: String = read!("{}\n");
            match item.to_lowercase().as_str() {
                "exit" | "quit" => break,
                _ => todo.add_item(item)
            };
        }
        
        println!("Addding the following todo:\n\n{}", todo.to_string());
        println!("Continue? (y/n) ");
        let is_ok: String = read!("{}\n");
    
        match is_ok.to_lowercase().as_str() {
            "y" | "yes" =>  {
                add_todo(&config.todofile, todo);
                break;
            },
            _ => println!("Re-running add command\n")
        };
    }
}

pub fn handle_rm_cmd(config: Config){
    if config.args.len() < 2 {
        eprintln!("{}\n", RM_SUBCMDS);
        return;
    }

    let subcommand = config.args.get(2).unwrap();

    match subcommand.as_str() {
        "all" | "-A" => remove_all_todos( &config.todofile),
        "primary" | "-P" => remove_primary_todo(&config.todofile),
        "title" | "-T" => remove_todo_by_title(&config.args, &config.todofile),
        _ => {
            eprintln!("Unknown command \"{}\"", subcommand);
            eprintln!("{}\n", RM_SUBCMDS);
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const FILENAME:&str = "test.tudu";

    #[test]
    fn test_get_todos_from_file(){
        let todos = get_todos(FILENAME);
        assert!(todos.len() > 0, "todos should not be empty");
    }
    
    #[test]
    fn test_parse_title(){
        let (title, priority) = parse_title("[This a test]!!!!!");
        assert_eq!(title, "[This a test]".to_string());
        assert_eq!(priority, 5);
    }

    #[test]
    fn test_get_todos_by_title(){
        let title = "[test_todo]";
        let todos = vec![Todo::new(title.to_string(), 3)];
        let todos = include_todos_by_title(&todos, &title);
        assert!(todos.len() > 0, "should not be empty");
        assert_eq!(todos.first().unwrap().title, title);
    }

    #[test]
    fn test_exclude_todos_by_title(){
        let title = "[test_todo]";
        let todos = vec![Todo::new(title.to_string(), 3)];
        let todos = exclude_todos_by_title(&todos, &title);
        assert!(todos.len() == 0, "should empty by excluding the [test_todo]");
    }
}
