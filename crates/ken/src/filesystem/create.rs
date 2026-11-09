use std::{fs::{File, create_dir, create_dir_all}, io::Result, path::PathBuf};

pub fn create(
    all: bool,
    directory: bool,
    path: PathBuf
) -> Result<()> {
    if directory {
        if all {
            create_dir_all(path)
        }
        else {
            create_dir(path)
        }
    }
    else {
        _ = File::create_new(path)?;

        Ok(())
    }
}