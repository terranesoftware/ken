use std::{fs, io::{Result, Write, stdout}, path::PathBuf};

pub fn read(path: PathBuf) -> Result<()> {
    stdout().write_all(&fs::read(path)?)
} 