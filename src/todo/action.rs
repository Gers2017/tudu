use crate::Config;
use crate::todo::{Todo, is_todo_title, parse_title};
use crate::files::{read_file, write_file};

const MISSING_TODO_ERR: &str = "❌ No todo with title";
const EMPTY_TODO_ERR: &str = "❌ Empty todo file. No todos in";

pub fn get_todos(filename: &str) -> Vec<Todo> {
    let text = read_file(filename).unwrap(); // raw text
    // only lines with content
    let lines: Vec<&str> = text.lines()
    .filter(|line| line.len() > 0)
    .map(|line| line.trim())
    .collect(); 

    let temp_lines = lines.clone();
    let titles = temp_lines.iter()
    .filter(|l| is_todo_title(l))
    .map(|l| parse_title(l.to_string()));
    
    let mut todos: Vec<Todo> = titles
    .map(|(title, priority)| Todo::new(title, priority))
    .collect();

    let mut index = 0;

    for line in &lines[1..] { // start at second line to add items to the first todo
        if is_todo_title(line){
            index += 1
        } else {
            todos[index].add_item(line.to_string());
        }
    }
    
    sort_todos(&mut todos);
    todos
}

fn sort_todos(todos: &mut Vec<Todo>){
    todos.sort_by(|a, b| b.priority.cmp(&a.priority));
}

fn todos_to_text(todos: &Vec<Todo>) -> String{
    if todos.is_empty() {
        return "".to_string();
    }
    let str_todos = todos.iter().map(|todo| todo.to_string()).collect::<Vec<_>>();
    return str_todos.join("\n");
}

pub fn print_all_todos(filename: &str) {
    let todos = get_todos(filename);
    println!("{}", todos_to_text(&todos));
}

pub fn print_primary_todo(filename: &str) {
    let todos: Vec<Todo> = get_todos(filename);
    if todos.is_empty() {
        eprintln!("{} {}\n", EMPTY_TODO_ERR, filename);
        return;
    }

    let first = todos.first().unwrap();
    println!("{}", first.to_string());
}

pub fn print_todo_by_title(args: &[String], filename: &str){
    if args.len() < 4 {
        eprintln!("missing <todo-title> parameter\nusage tudu get title <todo-title>");
        std::process::exit(1);
    }

    let title = args[3].clone();
    println!("❓ Searching by title {}...", title);

    let todos: Vec<Todo> = get_todos(filename);
    let selected_todos = todos.iter()
    .filter(|todo| todo.match_title(&title))
    .collect::<Vec<&Todo>>();
    
    if selected_todos.is_empty() {
        eprintln!("{} \"{}\"\n", MISSING_TODO_ERR, title);
        return;
    }

    let first = selected_todos.first().unwrap();
    println!("{}", first.to_string());
}

pub fn save_todos(todos: Vec<Todo>, filename: &str){
    let content = todos_to_text(&todos);
    let result = write_file(filename, content.as_str());
    if result.is_err() {
        eprintln!("{}", result.unwrap_err()); 
    } 
}

pub fn add_todo(config: Config, todo: Todo){
    let mut todos = get_todos(&config.todofile);
    todos.push(todo);
    sort_todos(&mut todos);
    save_todos(todos, &config.todofile);
}

pub fn remove_all_todos(filename: &str) {
    save_todos(vec![], &filename);
}

pub fn remove_primary_todo(filename: &str){
    let todos: Vec<Todo> = get_todos(filename);
    if todos.is_empty() {
        eprintln!("{} {}\n", EMPTY_TODO_ERR, filename);
        return;
    }

    let slice = &todos.as_slice()[1..];
    let todos_to_save = slice.to_vec();

    let first = todos.first().unwrap();
    println!("{} was removed\n", first.title);
    save_todos(todos_to_save, filename);
}

pub fn remove_todo_by_title(args: &[String], filename: &str){
    if args.len() < 4 {
        eprintln!("missing <todo-title> parameter\nusage tudu rm <todo-title>");
        std::process::exit(1);
    }

    let title = args[3].clone();

    println!("❓ Searching by title {}...", title);

    let todos = get_todos(filename);
    let todos_to_save = todos.clone().iter()
    .filter(|todo| {
        return !todo.match_title(&title);
    })
    .cloned()
    .collect::<Vec<Todo>>();

    for t in todos.iter() {
        if t.match_title(&title) {
            println!("{} was removed\n", t.title);
        }
    }
    
    if todos_to_save.len() == todos.len() {
        eprintln!("{} \"{}\"\n", MISSING_TODO_ERR, title);
        return;
    }
    
    save_todos(todos_to_save, filename);
}
