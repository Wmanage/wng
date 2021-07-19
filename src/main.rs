mod errors;
use errors::{handle_error, Result, Error};

fn main() {
    match try_main () {
        Ok(()) => {},
        Err(e) => handle_error(e),
    }
}

fn try_main() -> Result<()> {
    Ok(())
}
