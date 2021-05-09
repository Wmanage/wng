use std::env;
use std::io::Write;
use std::path::Path;
use std::{fs, fs::File};

use crate::{error, WngError};

pub fn create(directory: &str, path: Option<&str>, with_messages: bool) -> crate::Result<()> {
    let config_file = crate::get_config_file(path);
    let cfg_toml: toml::Value = toml::from_str(&fs::read_to_string(config_file)?)?;

    if !cfg_toml.as_table().unwrap().contains_key("name") {
        return Err(error!("Missing key in .wng.config: `name`"));
    } else if !cfg_toml.as_table().unwrap().contains_key("email") {
        return Err(error!("Missing key in .wng.config: `email`"));
    }

    let name = if directory == "." {
        env::current_dir()?.to_str().unwrap().to_owned()
    } else {
        directory.to_owned()
    };

    if !Path::new(directory).exists() {
        fs::create_dir_all(directory)?;
    }

    let main = r#"#include <stdio.h>
#include <stdlib.h>

int main(void) {
    printf("Hello, World !\n");
    return EXIT_SUCCESS;
}"#;

    if !cfg_toml.as_table().unwrap().contains_key("name") {
        return Err(error!("Missing key in .wng.config: `name`"));
    } else if !cfg_toml.as_table().unwrap().contains_key("email") {
        return Err(error!("Missing key in .wng.config: `email`"));
    }

    let project = &format!("[project]\nname = \"{}\"\nversion = \"0.1.0\"\nauthors = [\n\t\"{}\"\n]\ndependencies = []", name,
        format!("{} <{}>", cfg_toml["name"].as_str().unwrap_or("Unspecified"), cfg_toml["email"].as_str().unwrap_or("un@specified.com"))
    );

    let to_create = vec![
        "src/",
        "build/",
        "build/debug/",
        "build/release/",
        "build/debug/objects/",
        "build/release/objects/",
    ];

    for folder in to_create {
        fs::create_dir(format!("{}/{}", directory, folder))?;
        if with_messages {
            println!("[+] Successfully created `{}`", folder);
        }
    }

    let mut src_main = File::create(format!("{}/src/main.c", directory))?;
    src_main.write_all(main.as_bytes())?;
    if with_messages {
        println!("[+] Successfully created `src/main.c`");
    }

    let mut project_toml = File::create(format!("{}/project.toml", directory))?;
    project_toml.write_all(project.as_bytes())?;
    if with_messages {
        println!("[+] Successfully created `project.toml`");

        println!(
            "[+] Successfully created project `{}` in `{}`",
            name, directory
        );
    }

    Ok(())
}
