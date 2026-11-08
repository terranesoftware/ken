use std::{fs::File, io::Result, path::absolute};

pub(crate) fn create(path: String) -> Result<()> {
    // Turn the path absolute, doesn't care if the file exists already
    let path = absolute(path)?;
    
    // Creates file if it doesn't exist
    _ = File::create_new(path)?;

    Ok(())
}