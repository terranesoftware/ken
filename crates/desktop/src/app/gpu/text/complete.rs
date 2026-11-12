use glyphon::{TextAtlas, TextRenderer};
use wgpu::MultisampleState;

use crate::App;

impl App {
    pub(crate) fn text(&mut self) {
        // This check exists because its technically possible that the Text thread completes before the GPU field gets filled
        let Some(gpu) = &mut self.gpu else {
            return;
        };
        
        // Make sure both the SurfaceConfiguration and the Text are created, otherwise wait for the second call
        let (device, queue, Some(sc), Some(text)) = (&gpu.device, &gpu.queue, &gpu.sc, &mut gpu.text) else {
            return;
        };
        
        let mut ta = TextAtlas::new(device, queue, &text.cache, sc.format);
        
        let tr = TextRenderer::new(&mut ta, device, MultisampleState::default(), None);
        
        text.ta = Some(ta);
        text.tr = Some(tr);
    }
}