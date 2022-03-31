use std::env;
use tudu::*;
use tudu::files::*;

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
