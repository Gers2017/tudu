use std::env;
use tudu::*;
use tudu::todo::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    
    match config.subcommand.as_str() {
        "get" => handle_get_cmd(config),
        "add" => handle_add_cmd(config),
        "rm" => handle_rm_cmd(config),
        _ => eprintln!("{}", AVAILABLE_CMDS),
    }
}

fn handle_get_cmd(config: Config){
    if let Some(sub) = config.args.get(2) {
        let filename = config.todofile;
        match sub.as_str() {
            "all" => print_all_todos(&filename),
            "primary" => print_primary_todo(&filename),
            "title" => print_todo_by_title(&config.args, &filename),
            _ => {
                eprintln!("\"{}\"", sub);
                eprintln!("{}", GET_SUBCMDS);
            },
        }
    } else {
        eprintln!("{}", GET_SUBCMDS);
    }
}

fn handle_add_cmd(config: Config){
    
}

fn handle_rm_cmd(config: Config){
    if let Some(sub) = config.args.get(2) {
        match sub.as_str() {
            "title" => remove_todo_by_title(config),
            _ => {
                eprintln!("\"{}\"", sub);
                eprintln!("{}", RM_SUBCMDS);
            },
        }
    } else {
        eprintln!("{}", RM_SUBCMDS);
    }
}