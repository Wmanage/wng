use crate::{
    config::parse_file,
    error,
    errors::{Error, Result},
    project::project::{Project, ProjectType},
};
use std::{
    fs::{self, File},
    io::Write,
    process::Command,
};

pub fn create_project(name: &str) -> Result<Project> {
    let src = format!("{}/src", name);
    fs::create_dir_all(&src)
        .map_err(|e| Error(format!("Failed to create directory: {}: {}.", src, e)))?;

    let build = format!("{}/build", name);
    fs::create_dir_all(&build)
        .map_err(|e| Error(format!("Failed to create directory: {}: {}.", build, e)))?;

    let ketchfile = format!("{}/ketchfile", name);
    File::create(&ketchfile)
        .map_err(|e| Error(format!("Failed to create file: {}: {}.", ketchfile, e)))?
        .write_all(format!("(name {})\n(version 0.1.0)\n", name).as_bytes())
        .map_err(|e| Error(format!("Failed to write file: {}: {}.", ketchfile, e)))?;

    let main = format!("{}/main.c", src);
    File::create(&main)
        .map_err(|e| Error(format!("Failed to create file: {}: {}.", main, e)))?
        .write_all(b"#include <stdlib.h>\n\nint\nmain (void)\n{\n  return EXIT_SUCCESS;\n}\n")
        .map_err(|e| Error(format!("Failed to write file: {}: {}.", main, e)))?;

    Project::from_config(parse_file(ketchfile)?)
}

pub fn build_project() -> Result<()> {
    let project = Project::from_config(parse_file("./ketchfile")?)?;
    let files = read_dir("./src/")?.into_iter().filter(|f| f.ends_with(".c")).collect::<Vec<String>>();
    let mut objs = vec![];

    println!("\x1b[0;32m*\x1b[0m Compiling {}::{} ({} files)...", project.name, project.version, files.len());
    for file in files {
        let mut flags = project.flags.clone();
        if let ProjectType::Shared = project.ptype {
            flags.push("-fpic".to_string());
        }
        flags.push(format!("-std={}", project.standard));
        flags.extend(vec!["-c".to_string(), file.clone(), "-o".to_string()]);
        let built = format!("build/{}", file[6..] /* Skip `./src/` prefix */
                                    .replace("/", "_")
                                    .replace(".c", ".o"));
        objs.push(built.to_string());
        flags.push(built);
        println!("{} {}", &project.compiler, flags.join(" "));
        let status = Command::new(&project.compiler)
                        .args(&flags)
                        .status()
                        .map_err(|e| Error(format!("Failed to summon command: `{} {}`: {}", project.compiler, flags.join(" "), e)))?;
        if !status.success() {
            return error!("Aborting at first failed command.");
        }
    }

    let program = if let ProjectType::Static = project.ptype {
        "ar".to_string()
    } else {
        project.compiler
    };
    let mut args = objs.clone();

    match project.ptype {
        ProjectType::Binary => args.extend(vec!["-o".to_string(), project.name]),
        ProjectType::Static => {
            args = vec!["rcs".to_string()];
            args.extend(objs);
            args.push(format!("lib{}.a", project.name));
        }
        ProjectType::Shared => args.extend(vec!["-shared".to_string(), "-o".to_string(), format!("lib{}.so", project.name)]),
    }

    println!("{} {}", program, args.join(" "));

    let status = Command::new(&program)
                        .args(&args)
                        .status()
                        .map_err(|e| Error(format!("Failed to summon command: `{} {}`: {}", program, args.join(" "), e)))?;
    if !status.success() {
        return error!("Aborting at first failed command.");
    }

    Ok(())
}

fn read_dir(dir: &str) -> Result<Vec<String>> {
    let readdir = fs::read_dir(dir).map_err(|e| Error(format!("Failed to read directory: {}: {}.", dir, e)))?;
    let mut content = vec![];

    for entry in readdir {
        let entry = entry.map_err(|e| Error(format!("Failed to get directory entry: {}: {}.", dir, e)))?;
        let stringified = entry.path().to_string_lossy().to_string();

        if entry.path().is_dir() {
            content.extend(read_dir(&stringified)?);
        } else {
            content.push(stringified);
        }
    }
    Ok(content)
}
