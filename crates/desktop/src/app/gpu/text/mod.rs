mod complete;

use std::sync::Arc;

use glyphon::{Cache, TextAtlas, TextRenderer, Viewport};
use wgpu::Device;

pub(crate) struct Text {
    pub(crate) cache: Cache,
    pub(crate) ta: Option<TextAtlas>,
    pub(crate) tr: Option<TextRenderer>,
    pub(crate) viewport: Viewport
}

impl Text {
    pub(super) fn new(device: Arc<Device>) -> Self {
        let cache = Cache::new(&device);
        let viewport = Viewport::new(&device, &cache);
        
        Self {
            cache,
            ta: None,
            tr: None,
            viewport
        }
    }
}