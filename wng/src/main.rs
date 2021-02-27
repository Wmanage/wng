#![forbid(unsafe_code)]

use std::{path::Path, process::Command};

use build::{build, clean, run};
use clap::{App, Arg, SubCommand};
use config::{manually::manually, reinit::reinit, setup::setup};
use create::create;
use deps::{add_dep, remove_dep};
use install::{install, uninstall};
use is_executable::IsExecutable;
use wng_lib::*;

fn main() -> Result<()> {
    let matches = App::new("WNG")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("WNG is the W package manager, a package and projects manager for the C programming language.")
        .subcommand(SubCommand::with_name("install").about("Installs the project dependencies."))
        .subcommand(SubCommand::with_name("uninstall").about("Uninstalls the project dependencies."))
        .subcommand(SubCommand::with_name("clean").about("Cleans the build folder."))
        .subcommand(
            SubCommand::with_name("build")
                .arg(
                    Arg::with_name("release")
                        .long("--release")
                        .help("Specifies to build with the -O3 flag activated."),
                )
                .about("Builds the current project."),
        )
        .subcommand(
            SubCommand::with_name("test")
                .arg(
                    Arg::with_name("release")
                        .long("--release")
                        .help("Specifies to test in release mode.")
                )
                .about("Runs the project tests.")
        )
        .subcommand(
            SubCommand::with_name("run")
                .arg(
                    Arg::with_name("release")
                        .long("--release")
                        .help("Specifies to run with the -O3 flag activated."),
                )
                .about("Runs the current project."),
        )
        .subcommand(
            SubCommand::with_name("dependencies")
                .subcommand(
                    SubCommand::with_name("add")
                        .arg(
                            Arg::with_name("dependency")
                                .required(true)
                                .index(1)
                                .help("The dependency to add."),
                        )
                        .about("Add a dependency to the current project."),
                )
                .subcommand(
                    SubCommand::with_name("remove")
                        .arg(
                            Arg::with_name("dependency")
                                .required(true)
                                .index(1)
                                .help("The dependency to remove."),
                        )
                        .about("Remove a dependency from the current project."),
                ),
        )
        .subcommand(SubCommand::with_name("setup").about("Setup WNG."))
        .subcommand(
            SubCommand::with_name("config")
                .subcommand(
                    SubCommand::with_name("reinit").about("Reinitializes WNG configuration."),
                )
                .subcommand(
                    SubCommand::with_name("edit")
                        .arg(
                            Arg::with_name("key")
                                .required(true)
                                .index(1)
                                .help("The key to edit."),
                        )
                        .arg(
                            Arg::with_name("value")
                                .required(true)
                                .index(2)
                                .help("The value to assign to the key."),
                        )
                        .about("Edits wanager configuration."),
                )
                .about("Configures wanager."),
        )
        .subcommand(
            SubCommand::with_name("new")
                .arg(
                    Arg::with_name("path")
                        .required(true)
                        .index(1)
                        .help("The directory where the project will be created"),
                )
                .about("Creates a new wanager project."),
        )
        .arg(Arg::with_name("plugin")
                .short("-p")
                .long("--plugin")
                .takes_value(true)
                .help("The plugin to run (optional)."))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        create(matches.value_of("path").unwrap(), None, true)?;
    } else if let Some(matches) = matches.subcommand_matches("config") {
        if let Some(_) = matches.subcommand_matches("reinit") {
            reinit(None)?;
        } else if let Some(matches) = matches.subcommand_matches("edit") {
            manually(
                None,
                matches.value_of("key").unwrap(),
                matches.value_of("value").unwrap(),
            )?;
        }
    } else if let Some(_) = matches.subcommand_matches("setup") {
        setup(None, env!("CARGO_PKG_VERSION"))?;
    } else if let Some(matches) = matches.subcommand_matches("dependencies") {
        if let Some(matches) = matches.subcommand_matches("add") {
            add_dep(matches.value_of("dependency").unwrap(), true)?;
        } else if let Some(matches) = matches.subcommand_matches("remove") {
            remove_dep(matches.value_of("dependency").unwrap(), true)?;
        }
    } else if let Some(matches) = matches.subcommand_matches("build") {
        build(None, matches.is_present("release"), !matches.is_present("release"), false)?;
    } else if let Some(matches) = matches.subcommand_matches("run") {
        run(None, vec![], matches.is_present("release"), matches.is_present("release"),false)?;
    } else if let Some(_) = matches.subcommand_matches("clean") {
        clean()?;
    } else if let Some(_) = matches.subcommand_matches("install") {
        install()?;
    } else if let Some(_) = matches.subcommand_matches("uninstall") {
        uninstall()?;
    } else if matches.value_of("plugin").is_some() {

            let plugin_path =
                &format!("{}/.wng/wng-{}", home_dir().unwrap().to_str().unwrap(), matches.value_of("plugin").unwrap());
            let pathed = Path::new(plugin_path);
            if !pathed.exists() {
                return Err(error!("No files found matching `", plugin_path, "`"));
            }

            if !pathed.is_executable() {
                return Err(error!("The plugin is not an executable file !"));
            }

            Command::new(plugin_path)
                .status()?;
    } else if let Some(m) = matches.subcommand_matches("test") {
        run(None, vec![], m.is_present("release"),false, true)?;
    }

    Ok(())
}
