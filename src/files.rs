use std::{fs, io, env};
use std::error::Error;
use crate::utils::prompt;

type BoxResult<T> = Result<T,Box<dyn Error>>;

pub fn get_tudufiles() -> BoxResult<Vec<String>> {
    let mut tudufiles: Vec<String> = vec![];
    
    let entries = fs::read_dir(env::current_dir()?)?
    .map(|res| res.map(|e| e.path()))
    .collect::<Result<Vec<_>, io::Error>>()?;
    
    for p in entries {
        if p.extension().map_or(false, |ext| ext == "tudu") {
            let name = p.file_name().unwrap().to_owned();
            tudufiles.push(String::from(name.to_string_lossy()));
        }
    }
    
    Ok(tudufiles)
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
    let tudu_files = get_tudufiles().unwrap_or(vec![]);
    if tudu_files.is_empty() {
        return Err("Not a single file has the \".tudu\" extension in the current directory");
    }

    if tudu_files.len() == 1 {
        return Ok(tudu_files.first().unwrap().to_owned());
    }
    println!("Multiple tudu files detected");
    return Ok(prompt(tudu_files));
}
