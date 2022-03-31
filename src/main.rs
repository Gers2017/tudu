#![allow(unused)]
use std::env;
use tudu::*;
use tudu::todo::Todo;
use tudu::todo::action::*;
use tudu::files::*;
use text_io::read;

fn main() {
    let todofile = get_todofile();
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args, todofile);
    
    match config.subcommand.as_str() {
        "get" => handle_get_cmd(config),
        "add" => handle_add_cmd(config),
        "rm" => handle_rm_cmd(config),
        _ => eprintln!("{}\n", AVAILABLE_CMDS),
    }
}

fn handle_get_cmd(config: Config){
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

fn handle_add_cmd(config: Config){
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
                add_todo(config, todo);
                break;
            },
            _ => println!("Re-running add command\n")
        };
    }
}

fn handle_rm_cmd(config: Config){
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
