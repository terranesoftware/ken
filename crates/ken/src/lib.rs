mod error;
pub use error::Error;

mod files;
pub use files::Files;

pub mod filesystem;
pub use filesystem::Filesystem;

pub struct Ken {
    files: Files,
    filesystem: Filesystem
}

impl Ken {
    pub fn new() -> Self {
        Self {
            filesystem: Filesystem::new(),
            files: Files::new()
        }
    }
}