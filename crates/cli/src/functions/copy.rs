use std::{fs, io::Result, path::PathBuf};

pub(crate) fn copy(
    from: PathBuf,
    to: PathBuf
) -> Result<()> {
    // Have to append `from` to `to` to create the "path to be"
    let to = if to.is_dir() {
        to.join(from.file_name().unwrap())
    }
    else {
        to
    };
    
    _ = fs::copy(from, to)?;

    Ok(())
}