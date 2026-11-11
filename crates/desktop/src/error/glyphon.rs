use glyphon::{PrepareError, RenderError};

use crate::Error;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum GlyphonError {
    Prepare(PrepareError),
    Render(RenderError)
}

impl From<PrepareError> for Error {
    fn from(value: PrepareError) -> Self {
        Error::Glyphon(GlyphonError::Prepare(value))
    }
}

impl From<RenderError> for Error {
    fn from(value: RenderError) -> Self {
        Error::Glyphon(GlyphonError::Render(value))
    }
}