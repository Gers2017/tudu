use argh;
use std::path::PathBuf;
use std::process;
use text_io::read;
use tudu::files::*;
use tudu::todo::action::*;
use tudu::todo::Todo;
use tudu::{Commands, GetSubCommand, RmSubCommand, TuduCli};

fn main() {
    let todofile: PathBuf = get_tudu_filename().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let tudu: TuduCli = argh::from_env();
    match tudu.command {
        Commands::Get(get) => handle_get_cmd(&get, &todofile),
        Commands::Rm(rm) => handle_rm_cmd(&rm, &todofile),
        Commands::Add(_add) => handle_add_cmd(&todofile),
    }
}

pub fn handle_get_cmd(get: &GetSubCommand, todofile: &PathBuf) {
    if get.all {
        print_all_todos(todofile, get.sort_by_title, get.reversed);
    } else if get.primary {
        print_primary_todo(todofile);
    } else if let Some(ref title) = get.title {
        print_todo_by_title(title, todofile);
    }
}

pub fn handle_rm_cmd(rm: &RmSubCommand, todofile: &PathBuf) {
    if rm.all {
        println!("Are you sure about this? [Y/N]");
        let answer: String = read!("{}\n");
        match answer.to_lowercase().as_str() {
            "y" | "yes" => remove_all_todos(todofile),
            _ => println!("Operation aborted"),
        }
    } else if rm.primary {
        remove_primary_todo(todofile);
    } else if let Some(ref title) = rm.title {
        remove_todo_by_title(title, todofile);
    }
}

pub fn handle_add_cmd(todofile: &PathBuf) {
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
        print!("Continue? ");
        println!("Accept [A], Recreate [R], Quit [Q]");
        let answer: String = read!("{}\n");

        match answer.to_lowercase().as_str() {
            "a" => {
                add_todo(todofile, todo);
                break;
            }
            "r" => {
                println!("Recreating the todo\n");
                continue;
            }
            _ => break,
        };
    }
}
