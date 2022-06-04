use crate::utils::prompt;
use std::error::Error;
use std::{env, fs, io};

pub fn get_tudufiles_from_dir() -> Result<Vec<String>, Box<dyn Error>> {
    let entries = env::current_dir()?
        .read_dir()?
        .map(|res| res.map(|d| d.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    return Ok(entries
        .iter()
        .filter(|p| p.is_file() && p.extension().map_or(false, |ext| ext == "tudu"))
        .map(|p| String::from(p.file_name().unwrap().to_owned().to_string_lossy()))
        .collect::<Vec<_>>());
}

pub fn read_file(filename: &str) -> io::Result<String> {
    let contents = fs::read_to_string(filename)?;
    Ok(contents)
}

pub fn write_file(filename: &str, contents: &str) -> io::Result<()> {
    fs::write(filename, contents)?;
    Ok(())
}

pub fn delete_file(filename: &str) -> io::Result<()> {
    fs::remove_file(filename)?;
    Ok(())
}

pub fn get_tudu_filename() -> Result<String, &'static str> {
    let tudu_files = get_tudufiles_from_dir().unwrap_or(vec![]);
    if tudu_files.is_empty() {
        return Err("Not a single file has the \".tudu\" extension in the current directory");
    }

    if tudu_files.len() == 1 {
        return Ok(tudu_files.first().unwrap().to_owned());
    }
    println!("Multiple tudu files detected");
    return Ok(prompt(tudu_files));
}
