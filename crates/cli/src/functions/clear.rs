use std::{fs::{OpenOptions, create_dir, remove_dir_all}, io::Result};

pub(crate) fn clear(
    directory: bool,
    path: String
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