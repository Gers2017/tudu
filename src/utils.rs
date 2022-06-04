use text_io::read;

pub fn prompt(options: &Vec<String>) -> Option<usize> {
    for (i, item) in options.iter().enumerate() {
        println!("({}) {}", i, item);
    }

    let index: usize = read!();
    let choice = options.get(index);

    if choice.as_ref().is_none() {
        return None;
    }

    return Some(index);
}
