use glyphon::FontSystem;

use crate::{Error, Gpu, app::gpu::Text};

pub(super) enum UserEvent {
    FontSystem(FontSystem),
    Gpu(Result<Gpu, Error>),
    Text(Text)
}