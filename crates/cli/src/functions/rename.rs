use std::{fs, io::Result, path::PathBuf};

pub(crate) fn rename(
    from: PathBuf,
    to: PathBuf
) -> Result<()> {
    fs::rename(from, to)
}