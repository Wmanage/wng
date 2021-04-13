use crate::Result;
use fs_extra::dir::CopyOptions;
use git2::Repository;

use crate::{error, WngError};
use std::fs;
use std::path::Path;

pub fn uninstall() -> Result<()> {
    if !Path::new("project.toml").exists() {
        return Err(error!(
            "Cannot find `project.toml` in the current directory."
        ));
    }

    let prjct_toml: toml::Value = toml::from_str(&fs::read_to_string("project.toml")?)?;

    if !prjct_toml.as_table().unwrap().contains_key("project") {
        return Err(error!("Missing key in project.toml: `project`"));
    } else if !prjct_toml["project"]
        .as_table()
        .unwrap()
        .contains_key("dependencies")
    {
        return Err(error!("Missing key in project.toml: `project.dependencies`"));
    }

    if !Path::new("src").exists() {
        return Err(error!("Missing `src/` at the project root."));
    }

    let deps = prjct_toml["project"]["dependencies"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s.as_str().unwrap_or(""))
        .collect::<Vec<&str>>();

    println!(
        "{}Uninstalling{} {} dependencies ...",
        if cfg!(windows) { "" } else { "\x1b[0;32m" },
        if cfg!(windows) { "" } else { "\x1b[0m" },
        deps.len()
    );

    for dep in &deps {
        let splited = dep.split("/").collect::<Vec<&str>>();

        if splited.len() != 5 {
            return Err(error!("Invalid repository link: `{}`", dep));
        }

        let user = splited[3];
        let repo = splited[4];

        println!(
            "{}Uninstalling{} `{}`",
            if cfg!(windows) { "" } else { "\x1b[0;32m" },
            if cfg!(windows) { "" } else { "\x1b[0m" },
            &format!("{}/{}", user, repo),
        );

        if !Path::new(&format!("src/{}", repo)).exists() {
            return Err(
                error!("Cannot find `", repo, "` in src/.")
            )
        }

        fs::remove_dir_all(&format!("src/{}", repo))?;

        println!(
            "{}Successfully uninstalled{} `{}`",
            if cfg!(windows) { "" } else { "\x1b[0;32m" },
            if cfg!(windows) { "" } else { "\x1b[0m" },
            &format!("{}/{}", user, repo),
        );
    }

    println!(
        "{}Sucessfully uninstalled{} {} dependencies.",
        if cfg!(windows) { "" } else { "\x1b[0;32m" },
        if cfg!(windows) { "" } else { "\x1b[0m" },
        &deps.len()
    );

    Ok(())
}

pub fn install() -> Result<()> {
    if !Path::new("project.toml").exists() {
        return Err(error!(
            "Cannot find `project.toml` in the current directory."
        ));
    }

    let prjct_toml: toml::Value = toml::from_str(&fs::read_to_string("project.toml")?)?;

    if !prjct_toml.as_table().unwrap().contains_key("project") {
        return Err(error!("Missing key in project.toml: `project`"));
    } else if !prjct_toml["project"]
        .as_table()
        .unwrap()
        .contains_key("dependencies")
    {
        return Err(error!("Missing key in project.toml: `project.dependencies`"));
    }

    if !Path::new("src").exists() {
        return Err(error!("Missing `src/` at the project root."));
    }

    let deps = prjct_toml["project"]["dependencies"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s.as_str().unwrap_or(""))
        .collect::<Vec<&str>>();

    println!(
        "{}Installing{} {} dependencies ...",
        if cfg!(windows) { "" } else { "\x1b[0;32m" },
        if cfg!(windows) { "" } else { "\x1b[0m" },
        deps.len()
    );

    for dep in &deps {
        let splited = dep.split("/").collect::<Vec<&str>>();

        if splited.len() != 5 {
            return Err(error!("Invalid repository link: `{}`", dep));
        }

        let user = splited[3];
        let repo = splited[4];

        println!(
            "{}Installing{} `{}`",
            if cfg!(windows) { "" } else { "\x1b[0;32m" },
            if cfg!(windows) { "" } else { "\x1b[0m" },
            &format!("{}/{}", user, repo),
        );

        Repository::clone(dep, &format!("{}-{}", user, repo))?;

        let src = &format!("{}-{}/src", user, repo);

        if !Path::new(src).exists() {
            return Err(error!("No `src/` folder in the cloned repo. Aborting."));
        }
        fs_extra::dir::copy(src, "src/", &CopyOptions::new())?;
        fs::remove_dir_all(&format!("{}-{}", user, repo))?;
        fs::rename("src/src/", &format!("src/{}", repo))?;

        println!(
            "{}Successfully installed{} `{}`",
            if cfg!(windows) { "" } else { "\x1b[0;32m" },
            if cfg!(windows) { "" } else { "\x1b[0m" },
            &format!("{}/{}", user, repo),
        );
    }

    println!(
        "{}Sucessfully installed{} {} dependencies.",
        if cfg!(windows) { "" } else { "\x1b[0;32m" },
        if cfg!(windows) { "" } else { "\x1b[0m" },
        &deps.len()
    );

    Ok(())
}
