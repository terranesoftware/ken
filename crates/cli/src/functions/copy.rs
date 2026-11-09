use std::{fs, io::Result, path::PathBuf};

pub(crate) fn copy(
    from: PathBuf,
    to: PathBuf
) -> Result<()> {
    let to = if to.is_dir() {
        to.join(from.file_name().unwrap())
    }
    else {
        to
    };
    
    _ = fs::copy(from, to)?;

    Ok(())
}