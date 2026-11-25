use std::{collections::BTreeMap, io::Write, path::{Path, PathBuf}};

use landmine::Landmine;
use pecia::Buffer as Inner;
use walkdir::WalkDir;

use crate::{Buffer, Error};

pub struct Directories {
    pub active: Option<PathBuf>,
    pub map: DirectoriesMap
}

pub struct DirectoriesMap(BTreeMap<PathBuf, Buffer>);

impl Directories {
    pub fn new() -> Self {
        Self {
            active: None,
            map: DirectoriesMap(BTreeMap::new())
        }
    }
}

impl DirectoriesMap {
    pub fn get(
        &self,
        key: &Path
    ) -> Landmine<&Buffer> {
        self.0.get(key).into()
    }
    
    pub fn get_mut(
        &mut self,
        key: &Path
    ) -> Landmine<&mut Buffer> {
        self.0.get_mut(key).into()
    }
    
    pub fn insert(
        &mut self,
        key: PathBuf
    ) -> Result<Option<Buffer>, Error> {
        // Create the text buffer
        let mut inner = Inner::vector();
        
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
        inner.edit(0..0, &temp)?;
        
        Ok(self.0.insert(key, Buffer::new(inner)))
    }
    
    pub fn remove(
        &mut self,
        key: &Path
    ) {
        // Remove the parent and its children
        self.0.retain(|path, _| !path.starts_with(key));
    }
}