use text_io::read;

pub const AVAILABLE_CMDS: &str = "Available commands: \nget\nadd\nrm";
pub const GET_SUBCMDS: &str = "Available subcommands:\nall\nprimary\ntitle <todo-title>";
pub const RM_SUBCMDS: &str = "Available subcommands:\ntitle <todo-title>";

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
