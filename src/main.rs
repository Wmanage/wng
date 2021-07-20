mod config;
mod errors;
mod project;

use errors::Result;
use project::manager::{build_project, create_project};
use std::process::exit;

fn main() -> ! {
    match try_main() {
        Ok(()) => exit(0),
        Err(e) => eprintln!("ketch: {}", e.0),
    }
    exit(1);
}

fn try_main() -> Result<()> {
    build_project(true)
}
