use crate::utils::prompt;
use std::ffi::OsString;
use std::path::PathBuf;
use std::{env, io};

pub fn get_tudufiles_from_dir() -> io::Result<Vec<PathBuf>> {
    let tudu = OsString::from("tudu");
    return Ok(env::current_dir()?
        .read_dir()?
        .map(|res| res.map(|d| d.path()))
        .filter(|res| res.is_err() || res.as_ref().unwrap().extension() == Some(&tudu))
        .collect::<io::Result<Vec<_>>>()?);
}

pub fn path_to_display(path: &PathBuf) -> String {
    return path.display().to_string();
}

pub fn paths_to_options(paths: &Vec<PathBuf>) -> Vec<String> {
    return paths
        .iter()
        .cloned()
        .map(|p| path_to_display(&p))
        .collect::<Vec<_>>();
}

pub fn get_tudu_filename() -> Result<PathBuf, &'static str> {
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
    let options = paths_to_options(&files);

    let option_index = prompt(&options);
    if option_index.as_ref().is_none() {
        return Err("Error: invalid range");
    }

    let index = option_index.unwrap();

    return Ok(files[index].to_owned());
}
