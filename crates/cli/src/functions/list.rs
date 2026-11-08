use std::{fs::read_dir, io::Result, path::{Path, PathBuf}};

pub(crate) fn list(
    path: Option<PathBuf>,
    recurse: bool
) -> Result<()> {
    let path = path.as_deref().unwrap_or(Path::new("."));
    let mut directories = Vec::new();

    if recurse {
        // Print the current directory in blue
        println!("\x1b[34m{}:\x1b[0m", path.display());
    }

    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        
        println!("{}", path.display());
        
        if recurse && path.is_dir() {
            directories.push(path);
        }
    }

    println!();

    for directory in directories {
        list(Some(directory), true)?;
    }

    Ok(())
}