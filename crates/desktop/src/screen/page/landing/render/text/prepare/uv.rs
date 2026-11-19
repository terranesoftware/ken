use std::sync::Arc;

use glyphon::{Resolution, Viewport};
use wgpu::{Queue, Texture};

pub(super) fn uv(
    queue: &Arc<Queue>,
    texture: &Texture,
    viewport: &mut Viewport
) {
    viewport.update(queue, Resolution {
        height: texture.height(),
        width: texture.width()
    });
}