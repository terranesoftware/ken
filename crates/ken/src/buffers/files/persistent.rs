use std::{collections::BTreeMap, fs::read_to_string, path::{Path, PathBuf}};

use pecia::Buffer as Inner;

use crate::{Buffer, Error};

pub struct Persistent(BTreeMap<PathBuf, Buffer>);

impl Persistent {
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
        let mut inner = Inner::vector();
        
        let contents = read_to_string(&key)?;
        inner.edit(0..0, contents.as_bytes())?;
        
        Ok(self.0.insert(key, Buffer::new(inner)))
    }
    
    pub fn remove(
        &mut self,
        key: &Path
    ) {
        self.0.remove(key);
    }
}