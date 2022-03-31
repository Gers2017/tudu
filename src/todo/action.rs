use crate::Config;
use crate::todo::{Todo, is_todo_title, parse_title};
use crate::files::{read_file, write_file};

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
        eprintln!("❌ No todo with title {}", title);
    }

    if let Some(first) = selected_todos.first() {
        first.print();
    }
}

pub fn save_todos(todos: Vec<Todo>, filename: &str){
    let str_todos = todos.iter().map(|todo| todo.to_string()).collect::<Vec<_>>();
    let contents = str_todos.join("\n");
    let result = write_file(filename, contents.as_str());
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
            println!("Deleting todo {}", t.title);
        }
    }
    
    if todos_to_save.len() == todos.len() {
        eprintln!("❌ No todo with title {}", title);
        return;
    }
    
    save_todos(todos_to_save, filename);
}