use std::{fs, io::Result};

pub(crate) fn rename(
    new: String,
    old: String
) -> Result<()> {
    fs::rename(old, new)
}