use std::{fs, io::Result, path::PathBuf};

pub(crate) fn r#move(
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
    
    fs::rename(&from, &to)
}