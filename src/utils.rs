use text_io::read;

pub const AVAILABLE_CMDS: &str = "Available commands: \nget\nadd\nrm";
pub const GET_SUBCMDS: &str = "Available subcommands:\nall\nprimary\ntitle <todo-title>";
pub const RM_SUBCMDS: &str = "Available subcommands:\ntitle <todo-title>";

pub const GET_ALL_HELP: &str = "gets all the tasks for the current tudu file";
pub const GET_PRIMARY_HELP: &str = "gets the task with the highest priority";
pub const GET_TITLE_HELP: &str = "gets a task by title (case insensitive)";

pub const ADD_HELP: &str = "Interactibly adds a new task to current the tudu file";

pub const RM_ALL_HELP: &str = "removes all the tasks for the current tudu file";
pub const RM_PRIMARY_HELP: &str = "removes the task with the highest priority";
pub const RM_TITLE_HELP: &str = "removes a task by title (case insensitive)";

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
