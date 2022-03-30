pub mod cli_utils;
pub mod todo;

pub use crate::cli_utils::*;

pub struct Config {
    pub subcommand: String,
    pub args: Vec<String>,
    pub todofile: String,
}

use std::process;
impl Config {
    pub fn new(args: &[String]) -> Config {
        if args.len() < 2 {
            eprintln!("{}", AVAILABLE_CMDS);
            process::exit(1);
        }
        
        let subcommand = args[1].clone();    
        Config{subcommand, args: args.to_vec(), todofile: "index.tudu".to_string()}
    }
}
