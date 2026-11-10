use glyphon::FontSystem;

use crate::{Error, Gpu};

pub(super) enum UserEvent {
    FontSystem(FontSystem),
    Gpu(Result<Gpu, Error>)
}