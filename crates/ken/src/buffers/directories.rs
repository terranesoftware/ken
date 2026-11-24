use std::{collections::BTreeMap, io::Write, path::{Path, PathBuf}};

use pecia::Buffer;
use walkdir::WalkDir;

use crate::Error;

pub struct Directories {
    pub active: Option<PathBuf>,
    map: BTreeMap<PathBuf, Buffer>
}

impl Directories {
    pub fn new() -> Self {
        Self {
            active: None,
            map: BTreeMap::new()
        }
    }
    
    pub fn get(
        &self,
        key: &Path
    ) -> Option<&Buffer> {
        self.map.get(key)
    }
    
    pub fn insert(
        &mut self,
        key: PathBuf
    ) -> Result<Option<Buffer>, Error> {
        // Create the text buffer
        let mut buffer = Buffer::vector();
        
        // Create a temp buffer for ease
        let mut temp = Vec::new();
        // Walk through the directory entries and format it into the temp buffer
        for entry in WalkDir::new(&key).min_depth(1).sort_by_file_name() {
            let entry = entry?;
            
            let indent = "  ".repeat(entry.depth() - 1);
            let name = entry.file_name().to_string_lossy();
            let suffix = if entry.file_type().is_dir() { "/" } else { "" };
            
            writeln!(temp, "{}{}{}", indent, name, suffix)?;
        }
        // Remove trailing newline
        temp.pop();
        
        // Write the temp buffer into the text buffer
        buffer.edit(0..0, &temp)?;
        
        Ok(self.map.insert(key, buffer))
    }
    
    pub fn remove(
        &mut self,
        key: &Path
    ) {
        // Remove the parent and its children
        self.map.retain(|path, _| !path.starts_with(key));
    }
}