use std::sync::{Arc, RwLock};

use glyphon::{Cache, FontSystem, SwashCache, TextAtlas, TextRenderer, Viewport};
use wgpu::{Device, MultisampleState, Queue, TextureFormat};

pub(crate) struct Text {
    pub(crate) fs: Arc<RwLock<FontSystem>>,
    pub(crate) sc: Arc<RwLock<SwashCache>>,
    pub(crate) ta: RwLock<TextAtlas>,
    pub(crate) tr: RwLock<TextRenderer>,
    pub(crate) viewport: RwLock<Viewport>
}

impl Text {
    pub(super) fn init(device: &Arc<Device>) -> (
        Cache,
        RwLock<Viewport>
    ) {
        let cache = Cache::new(&device);
        let viewport = RwLock::new(Viewport::new(&device, &cache));
        
        (cache, viewport)
    }

    pub(super) fn complete(
        cache: Cache,
        fs: Arc<RwLock<FontSystem>>,
        device: Arc<Device>,
        queue: Arc<Queue>,
        sc: Arc<RwLock<SwashCache>>,
        tf: TextureFormat,
        viewport: RwLock<Viewport>
    ) -> Self {
        let mut ta = TextAtlas::new(&device, &queue, &cache, tf);
        
        let tr = TextRenderer::new(&mut ta, &device, MultisampleState::default(), None);

        let ta = RwLock::new(ta);
        let tr = RwLock::new(tr);
        
        Self {
            fs,
            sc,
            ta,
            tr,
            viewport
        }
    }
}