#![allow(unused)]
use std::env;
use tudu::*;
use tudu::todo::*;
use tudu::files::*;

fn main() {
    let todofile = get_todofile();
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args, todofile);
    
    match config.subcommand.as_str() {
        "get" => handle_get_cmd(config),
        "add" => handle_add_cmd(config),
        "rm" => handle_rm_cmd(config),
        _ => eprintln!("{}", AVAILABLE_CMDS),
    }
}

fn handle_get_cmd(config: Config){
    if let Some(subcommand) = config.args.get(2) {
        let filename = config.todofile;
        match subcommand.as_str() {
            "all" => print_all_todos(&filename),
            "primary" => print_primary_todo(&filename),
            "title" => print_todo_by_title(&config.args, &filename),
            _ => {
                eprintln!("\"{}\"", subcommand);
                eprintln!("{}", GET_SUBCMDS);
            },
        }
    } else {
        eprintln!("{}", GET_SUBCMDS);
    }
}

fn handle_add_cmd(config: Config){
    let mut todo = Todo::new("[Hello mother]".to_string(), 22);
    todo.add_item("refactor tudu code".to_string());
    todo.add_item("make a readme".to_string());
    add_todo(config, todo);
}

fn handle_rm_cmd(config: Config){
    if config.args.len() < 2 {
        eprintln!("{}", RM_SUBCMDS);
        return;
    }

    let subcommand = config.args.get(2).unwrap();

    match subcommand.as_str() {
        "title" => remove_todo_by_title(&config.args, &config.todofile),
        _ => {
            eprintln!("Unknown command \"{}\"", subcommand);
            eprintln!("{}", RM_SUBCMDS);
        },
    }
}