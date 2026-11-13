use std::sync::Arc;

use glyphon::{Cache, TextAtlas, TextRenderer, Viewport};
use wgpu::{Device, MultisampleState, Queue, TextureFormat};

pub(crate) struct Text {
    pub(super) cache: Cache,
    pub(super) ta: TextAtlas,
    pub(super) tr: TextRenderer,
    pub(super) viewport: Viewport
}

impl Text {
    pub(super) fn init(device: &Arc<Device>) -> (
        Cache,
        Viewport
    ) {
        let cache = Cache::new(&device);
        let viewport = Viewport::new(&device, &cache);
        
        (cache, viewport)
    }

    pub(super) fn complete(
        cache: Cache,
        device: Arc<Device>,
        queue: Arc<Queue>,
        tf: TextureFormat,
        viewport: Viewport
    ) -> Self {
        let mut ta = TextAtlas::new(&device, &queue, &cache, tf);
        
        let tr = TextRenderer::new(&mut ta, &device, MultisampleState::default(), None);
        
        Self {
            cache,
            ta,
            tr,
            viewport
        }
    }
}