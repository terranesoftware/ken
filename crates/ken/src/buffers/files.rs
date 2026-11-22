use std::{collections::BTreeMap, fs::read_to_string, path::{Path, PathBuf}};

use pecia::Buffer;

use crate::Error;

pub struct Files(BTreeMap<PathBuf, Buffer>);

impl Files {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
    
    pub fn get(
        &self,
        key: &Path
    ) -> Option<&Buffer> {
        self.0.get(key)
    }
    
    pub fn insert(
        &mut self,
        key: PathBuf
    ) -> Result<Option<Buffer>, Error> {
        let mut buffer = Buffer::vector();
        
        let contents = read_to_string(&key)?;
        buffer.edit(0..0, contents.as_bytes())?;
        
        Ok(self.0.insert(key, buffer))
    }
    
    pub fn remove(
        &mut self,
        key: &Path
    ) {
        self.0.remove(key);
    }
}