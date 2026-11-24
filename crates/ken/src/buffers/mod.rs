mod directories;
pub use directories::Directories;

pub mod files;
pub use files::{File, Files};

use pecia::Buffer as Inner;

pub struct Buffer {
    pub inner: Inner,
    pub cursor: (usize, usize) // (line, index)
}

impl Buffer {
    fn new(inner: Inner) -> Self {
        Self {
            inner,
            cursor: (0, 0)
        }
    }
}