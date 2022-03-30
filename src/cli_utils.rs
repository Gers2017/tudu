pub const AVAILABLE_CMDS: &str = "Available commands: \nget\nadd\nrm";
pub const GET_SUBCMDS: &str = "Available subcommands:\nall\nprimary\ntitle <todo-title>";
pub const RM_SUBCMDS: &str = "Available subcommands:\ntitle <todo-title>";

pub fn eprint_unknow_cmd(unknow_cmd: &str, available_cmds: &str){
    eprintln!("unknown command \"{}\"\n{}", unknow_cmd, available_cmds);
}

