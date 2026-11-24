pub mod buffers;
use std::path::PathBuf;

use buffers::{Directories, Files};

mod error;
pub use error::Error;

use crate::buffers::Drafts;

pub mod filesystem;

pub struct Ken {
    pub active: PathBuf,
    pub directories: Directories,
    pub drafts: Drafts,
    pub files: Files
}

impl Ken {
    pub fn new() -> Self {
        Self {
            active: PathBuf::new(),
            directories: Directories::new(),
            drafts: Drafts::new(),
            files: Files::new()
        }
    }
}