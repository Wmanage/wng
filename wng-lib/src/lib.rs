#![forbid(unsafe_code)]

pub mod config;
pub use dirs::home_dir;
pub mod create;
pub mod deps;
mod errors;
pub use errors::*;
pub mod build;
use std::path::PathBuf;
use std::fs;
pub mod install;

pub fn get_config_file(path: Option<&str>) -> String {
    let home_dir = dirs::home_dir().unwrap();
    let default = format!("{}/.wng.config", home_dir.to_str().unwrap());
    path.map(|x| x.to_owned()).unwrap_or(default)
}


pub fn see_dir(dirname: &PathBuf, o: bool, tests: bool) -> Result<Vec<PathBuf>> {
    let entries = fs::read_dir(dirname)?;
    let mut toret: Vec<PathBuf> = vec![];

    for entry in entries {
        let entry = entry?;

        if entry.path().is_dir() {
            toret.extend(see_dir(&entry.path().to_owned(), o, tests)?);
        } else if o {
            if entry.path().extension().unwrap().to_str().unwrap() == "o" {
                toret.push(entry.path().to_owned());
            }
        } else if tests {
            if entry.path().extension().unwrap().to_str().unwrap() == "c" && !entry.path().to_str().unwrap().ends_with("main.c") {
                toret.push(entry.path().to_owned());
            }
        } else {
            if entry.path().extension().unwrap().to_str().unwrap() == "c" && !entry.path().to_str().unwrap().ends_with("tests.c") {
                toret.push(entry.path().to_owned());
            }
        }
    }

    Ok(toret)
}
