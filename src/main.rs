mod config;
mod errors;
mod project;

use errors::Result;
use project::{manager::{build_project, create_project}, ProjectType};
use std::{process::exit, env};
use getopt_rs::getopt;

fn main() -> ! {
    match try_main() {
        Ok(()) => exit(0),
        Err(e) => eprintln!("ketch: {}", e.0),
    }
    exit(1);
}

fn help(command: Option<&str>) {
    if let Some(command) = command {
        match command {
            "new" => println!("Usage: ketch new NAME [OPTION]...
OPTIONS
    -s, --static    Create a static library project.
    -S, --shared    Create a shared library project.
        --help      Display this help and exit."),
            "build" => println!("Usage: ketch build [OPTION]
OPTIONS
    --release   Build with optimisation flags.
    --help      Display this help and exit."),
            _ => unreachable!(),
        }
    } else {
        println!("Usage: ketch COMMAND [OPTION]...
COMMANDS
    new PATH    Create a new ketch project at PATH.
    build       Build the project according to the `ketchfile`.

OPTIONS
    --help      Display this help and exit.
    --version   Display version information and exit.");
    }
}

fn handle_new(args: &mut Vec<String>) -> Result<()> {
    args.remove(0);
    let mut ptype = ProjectType::Binary;
    while let Some((opt, _)) = getopt(args, "Ss\n", &[('S', "shared"), ('s', "static"), ('\n', "help")]) {
        match opt {
            'S' => ptype = ProjectType::Shared,
            's' => ptype = ProjectType::Static,
            '\n' => {
                help(Some("new"));
                return Ok(());
            }
            _ => exit(1),
        }
    }
    if args.len() < 2 {
        error!("Missing argument: NAME.")
    } else {
        create_project(&args[1], ptype)?;
        Ok(())
    }
}
fn handle_build(args: &mut Vec<String>) -> Result<()> {
    args.remove(0);
    let mut release = false;
    while let Some((opt, _)) = getopt(args, "\n\r", &[('\n', "help"), ('\r', "release")]) {
        match opt {
            '\n' => {
                help(Some("build"));
                return Ok(());
            }
            '\r' => release = true,
            _ => exit(1),
        }
    }
    build_project(release)
}
fn try_main() -> Result<()> {
    let mut args = env::args().collect::<Vec<String>>();
    
    if let Some(cmd) = args.iter().nth(1) {
        match cmd.as_str() {
            "--help" => help(None),
            "--version" => println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            "new" => return handle_new(&mut args),
            "build" => return handle_build(&mut args),
            x => return error!("`{}` is not a valid commands. Type `ketch --help` for a list of commands.", x),
        }
    }

    Ok(())
}
