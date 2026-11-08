use std::{fs::{File, canonicalize}, io::Result};

pub(crate) fn clear(path: String) -> Result<()> {
    // Canonicalize guarantees the path exists already
    // Using the --overwrite flag without something to overwrite is an error
    let path = canonicalize(path)?;
    
    // Truncates file
    _ = File::create(path)?;

    Ok(())
}