pub mod buffers;
use buffers::{Directories, Files};

mod error;
pub use error::Error;

pub struct Ken {
    pub directories: Directories,
    pub files: Files,
}

impl Ken {
    pub fn new() -> Self {
        Self {
            directories: Directories::new(),
            files: Files::new()
        }
    }
}