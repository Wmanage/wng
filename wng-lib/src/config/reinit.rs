use std::path::Path;
use std::{fs, fs::File};

pub fn reinit(path: Option<&str>) -> crate::Result<()> {
    let config_file = crate::get_config_file(path);

    if Path::new(&config_file).exists() {
        fs::remove_file(&config_file)?;
    }

    File::create(&config_file)?;

    Ok(())
}
