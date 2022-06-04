pub mod action;

#[derive(Debug, Clone)]
pub struct Todo {
    pub title: String,
    pub priority: u32,
    pub items: Vec<String>,
}

impl Todo {
    pub fn new(mut title: String, priority: u32) -> Todo {
        if !title.starts_with("[") && !title.ends_with("]") {
            title = format!("[{}]", title);
        }
        Todo {
            title,
            priority,
            items: vec![],
        }
    }

    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
    }

    pub fn match_title(&self, title: &str) -> bool {
        trim_title(self.title.to_lowercase()) == trim_title(title.to_lowercase())
    }

    pub fn to_string(&self) -> String {
        let bangs = "!".repeat(*&self.priority as usize);
        let todos = &self
            .items
            .iter()
            .map(|i| format!("  {}", i))
            .collect::<Vec<_>>();
        return format!("{} {}\n{}\n", &self.title, bangs, todos.join("\n"));
    }
}

pub fn trim_title(title: String) -> String {
    title.trim_matches(|c| c == '[' || c == ']').to_string()
}

pub fn parse_title(line: &str) -> (String, u32) {
    let index = line.chars().position(|c| c == ']').unwrap();
    let title = &line[..index + 1];
    let priority = line.chars().filter(|c| c == &'!').count() as u32;
    (title.to_string(), priority)
}

fn is_todo_title(line: &str) -> bool {
    line.starts_with("[") && line.chars().filter(|ch| ch == &']').count() > 0
}
