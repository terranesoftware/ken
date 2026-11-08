use std::{fs, io::Result, path::PathBuf};

pub(crate) fn rename(
    new: PathBuf,
    old: PathBuf
) -> Result<()> {
    fs::rename(old, new)
}