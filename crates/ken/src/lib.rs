pub mod buffers;
use std::path::PathBuf;

use buffers::{Directories, Files};

mod error;
pub use error::Error;

pub struct Ken {
    pub active: PathBuf,
    pub directories: Directories,
    pub files: Files
}

impl Ken {
    pub fn new() -> Self {
        Self {
            active: PathBuf::new(),
            directories: Directories::new(),
            files: Files::new()
        }
    }
}