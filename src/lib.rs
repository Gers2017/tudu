pub mod files;
pub mod todo;
pub mod utils;

use argh;
use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Tudu cli, Top Command
pub struct TuduCli {
    #[argh(switch, short = 'j', description = "jumps")]
    pub jump: bool,

    #[argh(subcommand)]
    pub command: Commands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Commands {
    Get(GetSubCommand),
    Rm(RmSubCommand),
    Add(AddSubCommand),
}

/// Get Subcommand
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "get")]
pub struct GetSubCommand {
    /// get all tudus
    #[argh(switch, short = 'a')]
    pub all: bool,

    /// sorting by title tudus
    #[argh(switch)]
    pub sort_by_title: bool,

    /// should reverse tudus
    #[argh(switch, short = 'r')]
    pub reversed: bool,

    /// get tudu by priority
    #[argh(switch, short = 'p')]
    pub primary: bool,
    /// get tudu by title
    #[argh(option, short = 't')]
    pub title: Option<String>,
}

/// Rm Subcommand
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "rm")]
pub struct RmSubCommand {
    /// remove all tudus
    #[argh(switch, short = 'a')]
    pub all: bool,
    /// remove tudu by priority
    #[argh(switch, short = 'p')]
    pub primary: bool,
    /// remove tudu by title
    #[argh(option, short = 't')]
    pub title: Option<String>,
}

/// Add Subcommand
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "add")]
pub struct AddSubCommand {}

#[cfg(test)]
mod tests {
    use crate::todo::action::*;
    use crate::todo::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_get_todos_from_file() {
        let todo_file = PathBuf::from("__test_get_todos.tudu");

        let _todos = vec![
            Todo::new("[Bar]".to_string(), 10),
            Todo::new("[Foo]".to_string(), 1),
        ];

        save_todos(&_todos, &todo_file).expect("Should create a todo file");
        let todos = get_todos_from_file(&todo_file);
        assert!(!todos.is_empty(), "Should not be empty");
        assert_eq!(todos[0].title, "[Bar]");
        assert_eq!(todos[0].priority, 10);

        assert!(
            fs::remove_file(&todo_file).is_ok(),
            "should delete todo file"
        );
    }

    #[test]
    fn test_save_todos() {
        let todo_file = PathBuf::from("__test_save_todos.tudu");

        let _todos = vec![
            Todo::new("[Bar]".to_string(), 10),
            Todo::new("[Foo]".to_string(), 1),
        ];

        save_todos(&_todos, &todo_file).expect("Should create a todo file");

        remove_primary_todo(&todo_file);

        let primary = get_primary_todo(&todo_file);
        match primary {
            Some(todo) => debug_assert_ne!(todo.title.as_str(), "[Bar]"),
            None => eprintln!("Empty primary todo"),
        }

        assert!(
            fs::remove_file(&todo_file).is_ok(),
            "should delete todo file"
        );
    }

    #[test]
    fn test_parse_title() {
        let (title, priority) = parse_title("[This a test]!!!!!");
        assert_eq!(title, "[This a test]".to_string());
        assert_eq!(priority, 5);
    }

    #[test]
    fn test_title_match() {
        let todo = Todo::new("MY_TEST_TODO".to_string(), 0);
        let expect_match = "my_test_todo";
        assert!(todo.match_title(&expect_match));
    }

    #[test]
    fn test_get_todos_by_title() {
        let title = "[test_todo]";
        let todos = vec![Todo::new(title.to_string(), 3)];
        let todos = filter_by_title(&todos, &title);
        assert!(!todos.is_empty(), "should not be empty");
        assert_eq!(todos.first().unwrap().title, title);
    }
}
