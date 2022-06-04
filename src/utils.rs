use text_io::read;

pub const AVAILABLE_CMDS: &str = "Available commands: [ get, add, rm ]";
pub const GET_SUBCMDS: &str = "Available subcommands: [ all, primary, title <todo-title> ]";
pub const RM_SUBCMDS: &str = "Available subcommands: [ all, primary, title <todo-title> ]";
pub const ADD_SUBCMDS: &str = "Interactively adds a new task to current the tudu file";

pub const GET_ALL_HELP: &str = "Gets all the tasks for the current tudu file";
pub const GET_PRIMARY_HELP: &str = "Gets the task with the highest priority";
pub const GET_TITLE_HELP: &str = "Gets a task by title (case insensitive)";

pub const RM_ALL_HELP: &str = "Removes all the tasks for the current tudu file";
pub const RM_PRIMARY_HELP: &str = "Removes the task with the highest priority";
pub const RM_TITLE_HELP: &str = "Removes a task by title (case insensitive)";

pub const MISSING_TODO_TITLE_ERR: &str = "Missing <todo-title> parameter after title subcommand";

pub fn prompt(options: Vec<String>) -> String {
    for (i, word) in options.iter().enumerate() {
        println!("({}) {}", i, word);
    }

    print!("\nPlease select one\n");
    let i: usize = read!();
    let word = options.get(i);

    if word.as_ref().is_none() {
        eprintln!("Invalid range");
        std::process::exit(1);
    }

    return word.unwrap().to_owned();
}
