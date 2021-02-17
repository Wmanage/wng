use crate::{error, see_dir, Result, WngError};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
    time::Instant,
};

pub fn run(path: Option<&str>, args: Vec<String>, release: bool, test: bool) -> Result<()> {
    let name = build(path, release, test)?;
    if release {
        let status = Command::new(&format!("./build/release/{}", name))
            .args(&args)
            .status()?;
        if !status.success() {
            return Err(error!(
                "Run failed, exit code:",
                (status.code().unwrap_or(-1))
            ));
        }
    } else {
        let status = Command::new(&format!("./build/debug/{}", name))
            .args(&args)
            .status()?;
        if !status.success() {
            return Err(error!(
                "Run failed, exit code:",
                (status.code().unwrap_or(-1))
            ));
        }
    }

    Ok(())
}

pub fn clean() -> Result<()> {
    fs::remove_dir_all("build")?;
    let to_create = vec![
        "build/",
        "build/debug/",
        "build/release/",
        "build/debug/objects/",
        "build/release/objects/",
    ];

    for folder in to_create {
        if !Path::new(folder).exists() {
            fs::create_dir(format!("{}", folder))?;
        }
    }

    Ok(())
}

pub fn build(path: Option<&str>, release: bool, test: bool) -> Result<String> {
    let config_file = crate::get_config_file(path);
    let cfg_toml: toml::Value = toml::from_str(&fs::read_to_string(config_file)?)?;

    if !cfg_toml.as_table().unwrap().contains_key("cc") {
        return Err(error!("Missing key in .wng.config: `name`"));
    }

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
        .contains_key("version")
    {
        return Err(error!("Missing key in project.toml: `project.version`"));
    } else if !prjct_toml["project"]
        .as_table()
        .unwrap()
        .contains_key("name")
    {
        return Err(error!("Missing key in project.toml: `project.name`"));
    }

    if !Path::new("src").exists() {
        return Err(error!("Missing `src/` at the project root."));
    }

    let to_create = vec![
        "build/",
        "build/debug/",
        "build/release/",
        "build/debug/objects/",
        "build/release/objects/",
    ];

    for folder in to_create {
        if !Path::new(folder).exists() {
            fs::create_dir(format!("{}", folder))?;
        }
    }

    let files = see_dir(&PathBuf::from_str("src/").unwrap(), false, test)?;

    if test {
        clean()?;
    }

    let cc = cfg_toml["cc"].as_str().unwrap_or("gcc");

    println!(
        "{} {} v{}",
        if cfg!(windows) {
            "Compiling"
        } else {
            "\x1b[0;32mCompiling\x1b[0m"
        },
        prjct_toml["project"]["name"].as_str().unwrap(),
        prjct_toml["project"]["version"].as_str().unwrap()
    );

    let start = Instant::now();
    for file in files {
        let object_name = if release {
            format!(
                "build/release/objects/{}.o",
                file.to_str()
                    .unwrap()
                    .replace("src/", "")
                    .replace("/", "-")
                    .replace(".c", "")
            )
        } else {
            format!(
                "build/debug/objects/{}.o",
                file.to_str()
                    .unwrap()
                    .replace("src/", "")
                    .replace("/", "-")
                    .replace(".c", "")
            )
        };

        let status = if release {
            Command::new(cc)
                .arg("-c")
                .arg(file.to_str().unwrap())
                .arg("-o")
                .arg(object_name)
                .arg("-O3")
                .arg("-W")
                .arg("-Wall")
                .arg("-Werror")
                .arg("-Wextra")
                .status()?
        } else {
            Command::new(cc)
                .arg("-c")
                .arg(file.to_str().unwrap())
                .arg("-o")
                .arg(object_name)
                .arg("-W")
                .arg("-Wall")
                .arg("-Werror")
                .arg("-Wextra")
                .status()?
        };

        if !status.success() {
            return Err(if cfg!(windows) {
                error!("Compilation failed")
            } else {
                error!("\x1b[0;31mCompilation failed\x1b[0m")
            });
        }
    }

    let objects = if release {
        see_dir(&PathBuf::from_str("build/release/objects").unwrap(), true, test)?
    } else {
        see_dir(&PathBuf::from_str("build/debug/objects").unwrap(), true, test)?
    };

    let comp_status = if release {
        Command::new(cc)
            .args(objects)
            .arg("-o")
            .arg(&format!(
                "build/release/{}",
                prjct_toml["project"]["name"].as_str().unwrap()
            ))
            .arg("-O3")
            .arg("-W")
            .arg("-Wall")
            .arg("-Werror")
            .arg("-Wextra")
            .status()?
    } else {
        Command::new(cc)
            .args(objects)
            .arg("-o")
            .arg(&format!(
                "build/debug/{}",
                prjct_toml["project"]["name"].as_str().unwrap()
            ))
            .arg("-W")
            .arg("-Wall")
            .arg("-Werror")
            .arg("-Wextra")
            .status()?
    };

    if !comp_status.success() {
        return Err(if cfg!(windows) {
            error!("Compilation failed")
        } else {
            error!("\x1b[0;31mCompilation failed\x1b[0m")
        });
    }

    let elapsed = start.elapsed();

    if release {
        if cfg!(windows) {
            println!(
                "Finished release [optimized] in {:.2}s",
                elapsed.as_secs_f32()
            );
        } else {
            println!(
                "\x1b[0;32mFinished\x1b[0m release [optimized] in {:.2}s",
                elapsed.as_secs_f32()
            );
        }
    } else {
        if cfg!(windows) {
            println!(
                "Finished dev [unoptimized] in {:.2}s",
                elapsed.as_secs_f32()
            );
        } else {
            println!(
                "\x1b[0;32mFinished\x1b[0m dev [unoptimized] in {:.2}s",
                elapsed.as_secs_f32()
            );
        }
    }

    Ok(prjct_toml["project"]["name"].as_str().unwrap().to_owned())
}
