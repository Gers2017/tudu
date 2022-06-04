use crate::utils::prompt;
use std::ffi::OsString;
use std::{env, io};

pub fn get_tudufiles_from_dir() -> io::Result<Vec<String>> {
    let tudu = OsString::from("tudu");
    let entries = env::current_dir()?
        .read_dir()?
        .map(|res| res.map(|d| d.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    return Ok(entries
        .iter()
        .filter(|p| p.is_file() && p.extension() == Some(&tudu))
        .map(|p| String::from(p.file_name().unwrap().to_owned().to_string_lossy()))
        .collect::<Vec<_>>());
}

pub fn get_tudu_filename() -> Result<String, &'static str> {
    let get_files_result = get_tudufiles_from_dir();
    if let Err(ref _err) = get_files_result {
        return Err("Error at getting tudu files from current directory");
    }

    let files = get_files_result.unwrap();
    if files.is_empty() {
        return Err("Not a single file has the \".tudu\" extension in the current directory");
    }

    if files.len() == 1 {
        return Ok(files.first().unwrap().to_owned());
    }

    println!("Please choose a file");
    let options = files.clone();

    let option_index = prompt(&options);
    if option_index.as_ref().is_none() {
        return Err("Error: invalid range");
    }

    let index = option_index.unwrap();

    return Ok(files[index].to_owned());
}
