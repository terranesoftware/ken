use std::io;

use pecia::buffers::BufferError;

#[derive(Debug)]
pub enum Error {
    Buffer(BufferError),
    Io(io::Error),
    WalkDir(walkdir::Error)
}

impl From<BufferError> for Error {
    fn from(value: BufferError) -> Self {
        Self::Buffer(value)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<walkdir::Error> for Error {
    fn from(value: walkdir::Error) -> Self {
        Self::WalkDir(value)
    }
}