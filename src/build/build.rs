use lines_from_file::lines_from_file;
use serde_json::*;
use std::path::Path;
use std::process::Command;
use std::io::ErrorKind;

pub fn removebinary() {
    match std::fs::remove_file("build/debug/debug.exe") {
        Ok(_) => (),
        Err(e) => {

            // Because if it is equal to NotFound that would tell that the file hasn't been compiled
            if e.kind() == ErrorKind::NotFound {
                std::process::exit(-1);
            }
            eprintln!("{:?}", e.kind());
            std::process::exit(58);
        }
    };
}

pub fn build() {
    if !Path::new("deps.dat").exists() {
        eprintln!("File `deps.dat` not found. Make sure to be in a wanager project");
        std::process::exit(69);
    }

    let lines: Vec<String> = lines_from_file("deps.dat");
    let mut files: Vec<String> = Vec::new();
    for i in 0..lines.len() {
        files.push(format!("src\\{}\\*.c", lines[i]));
    }

    Command::new("gcc")
        .arg("src/*.c")
        .args(files)
        .arg("-o")
        .arg("build/debug/debug.exe")
        .arg("-W")
        .arg("-Wall")
        .arg("-Werror")
        .arg("-Wextra")
        .status()
        .expect("Error while running compilation command.");
}
pub fn buildhard() {
    let lines: Vec<String> = lines_from_file("deps.dat");
    let mut files: Vec<String> = Vec::new();
    for i in 0..lines.len() {
        files.push(format!("src\\{}\\*.c", lines[i]));
    }

    Command::new("gcc")
        .arg("src/*.c")
        .args(files)
        .arg("-o")
        .arg("build/release/release.exe")
        .status()
        .expect("Error while running compilation command.");
}
pub fn buildcustom() {
    if !Path::new("build.py").exists() {
        eprintln!("Build script not found");
        std::process::exit(64);
    }
    let content = lines_from_file("project.json").join("\n");
    let json: Value = match serde_json::from_str(&content) {
        Ok(j) => j,
        Err(_e) => {
            eprintln!("Failed to parse project.json");
            std::process::exit(66);
        }
    };

    if json["pyinterpreter"] == Value::Null {
        let ver = Command::new("python")
            .arg("--version")
            .output()
            .expect("Failed to get python version");
        let messagechars: Vec<char> = std::str::from_utf8(&ver.stdout).unwrap().chars().collect();

        if messagechars[7] < '3' && messagechars[9] < '5' {
            eprintln!("Python version has to be 3.5 or newer");
            std::process::exit(65);
        }
        Command::new("python")
            .arg("build.py")
            .status()
            .expect("Failed to run build script");
    } else {
        let pypath = match &json["pyinterpreter"] {
            Value::String(s) => s,
            _ => {
                eprintln!("Pyinterpreter has to be a valid string");
                std::process::exit(67);
            }
        };

        let ver = Command::new(pypath)
            .arg("--version")
            .output()
            .expect("Failed to get python version");
        let messagechars: Vec<char> = std::str::from_utf8(&ver.stdout).unwrap().chars().collect();

        if messagechars[7] < '3' && messagechars[9] < '5' {
            eprintln!("Python version has to be 3.5 or newer");
            std::process::exit(65);
        }
        Command::new(pypath)
            .arg("build.py")
            .status()
            .expect("Failed to run build script");
    }
}