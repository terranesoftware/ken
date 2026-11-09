use std::{fs::{OpenOptions, create_dir, remove_dir_all}, io::Result, path::PathBuf};

pub fn clear(
    directory: bool,
    path: PathBuf
) -> Result<()> {
    if directory {
        remove_dir_all(&path)?;

        create_dir(path)
    }
    else {
        _ = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)?;

        Ok(())
    }
}