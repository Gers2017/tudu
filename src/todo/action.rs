use crate::todo::*;
use std::{fs, io};

const MISSING_TODO_ERR: &str = "❌ No todo with title";
const EMPTY_TODO_ERR: &str = "❌ Empty todo file. No todos in";

pub fn get_todos_from_file(todofile: &str) -> Vec<Todo> {
    let read_res = fs::read_to_string(todofile);
    if let Err(ref err) = read_res {
        eprint!("{}", err);
        return vec![];
    }

    let text = read_res.unwrap();
    let lines: Vec<&str> = text
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| line.trim())
        .collect();

    if lines.is_empty() {
        return vec![];
    }

    let temp_lines = lines.clone();
    let titles = temp_lines
        .iter()
        .filter(|l| is_todo_title(l))
        .map(|l| parse_title(l.as_ref()));

    let mut todos: Vec<Todo> = titles
        .map(|(title, priority)| Todo::new(title, priority))
        .collect();

    let mut index = 0;

    for line in &lines[1..] {
        // start at second line to add items to the first todo
        if is_todo_title(line) {
            index += 1
        } else {
            todos[index].add_item(line.to_string());
        }
    }

    sort_todos_by_priority(&mut todos);
    return todos;
}

pub fn sort_todos_by_priority(todos: &mut Vec<Todo>) {
    todos.sort_by(|a, b| b.priority.cmp(&a.priority));
}

pub fn sort_todos_by_title(todos: &mut Vec<Todo>) {
    todos.sort_by(|a, b| a.title.cmp(&b.title));
}

pub fn todos_to_text(todos: &Vec<Todo>) -> String {
    if todos.is_empty() {
        return "".to_string();
    }
    return todos
        .iter()
        .map(|todo| todo.to_string())
        .collect::<Vec<_>>()
        .join("\n");
}

pub fn filter_by_title(todos: &Vec<Todo>, title: &str) -> Vec<Todo> {
    return todos
        .iter()
        .filter(|todo| todo.match_title(title))
        .cloned()
        .collect::<Vec<Todo>>();
}

pub fn save_todos(todos: &Vec<Todo>, todofile: &str) -> io::Result<()> {
    return fs::write(todofile, todos_to_text(todos).as_str());
}

pub fn get_primary_todo(todofile: &str) -> Option<Todo> {
    let todos = get_todos_from_file(todofile);
    return todos.first().map(|t| t.to_owned());
}

pub fn get_todo_by_title(title: &str, todofile: &str) -> Option<Todo> {
    let todos = get_todos_from_file(todofile)
        .iter()
        .filter(|res| res.match_title(title))
        .cloned()
        .collect::<Vec<_>>();
    return todos.first().map(|t| t.to_owned());
}

pub fn print_all_todos(todofile: &str, sort_by_title: bool, reversed: bool) {
    let mut tudus = get_todos_from_file(todofile);
    if sort_by_title {
        sort_todos_by_title(&mut tudus);
    }

    if reversed {
        tudus.reverse()
    }

    println!("{}", todos_to_text(&tudus));
}

pub fn print_primary_todo(todofile: &str) {
    let primary = get_primary_todo(todofile);
    match primary {
        Some(todo) => println!("{}", todo.to_string()),
        None => eprintln!("{} {}\n", EMPTY_TODO_ERR, todofile),
    }
}

pub fn print_todo_by_title(title: &str, todofile: &str) {
    println!("❓ Searching by title \"{}\"", title);

    match get_todo_by_title(title, todofile) {
        Some(todo) => println!("{}", todo.to_string()),
        None => eprintln!("{} \"{}\"\n", MISSING_TODO_ERR, title),
    }
}

pub fn add_todo(todofile: &str, todo: Todo) {
    let mut todos = get_todos_from_file(todofile);
    todos.push(todo);
    sort_todos_by_priority(&mut todos);
    save_todos(&todos, todofile).unwrap_or_else(|err| eprintln!("{}", err));
}

pub fn remove_all_todos(todofile: &str) {
    fs::write(todofile, "").unwrap_or_else(|err| eprintln!("{}", err));
}

pub fn remove_primary_todo(todofile: &str) {
    let mut todos: Vec<Todo> = get_todos_from_file(todofile);

    if todos.is_empty() {
        eprintln!("{} {}\n", EMPTY_TODO_ERR, todofile);
        return;
    }

    let removed_todo = todos.remove(0);
    println!("{} tudu was removed\n", removed_todo.title);

    save_todos(&todos, todofile).unwrap_or_else(|err| eprintln!("{}", err));
}

pub fn remove_todo_by_title(title: &str, todofile: &str) {
    println!("❓ Searching by title {}...", title);

    let mut todos = get_todos_from_file(todofile);
    let indexes = todos
        .iter()
        .enumerate()
        .filter(|res| res.1.match_title(title))
        .map(|res| res.0)
        .collect::<Vec<_>>();

    if indexes.is_empty() {
        eprintln!("{} \"{}\"\n", MISSING_TODO_ERR, title);
        return;
    }

    println!("Removing todo of title: {}", title);
    todos.remove(indexes[0]);
    save_todos(&todos, todofile).unwrap_or_else(|err| eprintln!("{}", err));
}
