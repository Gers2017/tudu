use std::fs;
use std::io;
use crate::Config;

#[derive(Debug, Clone)]
pub struct Todo {
    pub title: String,
    pub priority: u32,
    pub items: Vec<String>,
}

impl Todo {
    pub fn new(title: String, priority: u32) -> Todo{
        Todo{title, priority, items: vec![]}
    }

    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
    }

    pub fn print(&self) {
        println!("{} ❗ priority: {}", self.title, self.priority);
        for i in &self.items{
            println!("  {}", i);
        }
    }
}

// move to utils
pub fn read_file(filename: &str) -> io::Result<String> {
    let contents = fs::read_to_string(filename)?;
    Ok(contents)
}

fn parse_title(line: String) -> (String, u32) {
    let index = line.chars().position(|c| c == ']').unwrap();
    let title = &line[..index + 1];
    let priority = line.chars().filter(|c| c == &'!').count() as u32;
    (title.to_string(), priority)
}


fn is_todo_title(line: &str) -> bool {
    line.starts_with("[") && line.chars().filter(|ch| ch == &']').count() > 0
}

pub fn get_todos(filename: &str) -> Vec<Todo> {
    let text = read_file(filename).unwrap(); // raw text
    let lines: Vec<&str> = text.lines().filter(|line| line.len() > 0).map(|line| line.trim()).collect(); // only lines with content
    let temp_lines = lines.clone();
    let titles = temp_lines.iter().filter(|l| is_todo_title(l)).map(|l| parse_title(l.to_string()));
    let mut todos: Vec<Todo> = titles.map(|(title, priority)| Todo::new(title, priority)).collect();

    let mut index = 0;

    for line in &lines[1..] { // start at second line to add items to the first todo
        if is_todo_title(line){
            index += 1
        } else {
            todos[index].add_item(line.to_string());
        }
    }

    todos.sort_by(|a, b| b.priority.cmp(&a.priority));
    todos
}

pub fn save_todo(title: String, priority: u32, items: Vec<String>) {
    let mut todo = Todo::new(title, priority);
    todo.items = items;
    // append to text file
}

fn trim_title(title: String) -> String {
    title.trim_matches(|c| c == '[' || c == ']').to_string()
}

pub fn print_all_todos(filename: &str) {
    let todos = get_todos(filename);
    for t in  todos {
        t.print();
    }
}

pub fn print_primary_todo(filename: &str) {
    let todos: Vec<Todo> = get_todos(filename);
    if let Some(first) = todos.first() {
        first.print();
    }
}

pub fn print_todo_by_title(args: &[String], filename: &str){
    if args.len() < 4 {
        eprintln!("missing <todo-title> parameter\nusage get title <todo-title>");
        std::process::exit(1);
    }
    // cli_utils::validate_args(args, 4, "missing <todo-title> parameter\nusage get title <todo-title>"); // tudu get title <todo-title> error, print usage

    let title = args[3].clone();
    println!("❓ Searching by title {}...", title);

    let todos: Vec<Todo> = get_todos(filename);
    
    let selected_todos: Vec<&Todo> = todos.iter().filter(|todo| trim_title(todo.title.to_lowercase()) == title.to_lowercase()).collect();

    if selected_todos.is_empty() {
        eprintln!("❌ No todo with title {}", title);
    }

    if let Some(first) = selected_todos.first() {
        first.print();
    }
}


pub fn add_todo(config: Config){
    println!("TODO {}", config.subcommand);
}

pub fn remove_todo_by_title(config: Config){
    println!("TODO {}", config.subcommand);
}