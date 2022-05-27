use std::{env, process};
use tudu::*;
use tudu::files::*;

fn main() {
    let todofile = get_tudu_filename().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let config = Config::new(env::args(), todofile).unwrap_or_else(|err| {
        eprintln!("{}\n{}", err, AVAILABLE_CMDS);
        process::exit(1);
    });
    
    match config.subcommand.as_str() {
        "get" => handle_get_cmd(config),
        "add" => handle_add_cmd(config),
        "rm" => handle_rm_cmd(config),
        _ => eprintln!("{}\n", AVAILABLE_CMDS),
    }
}
