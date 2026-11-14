use std::sync::RwLock;

use glyphon::{TextAtlas, TextRenderer, Viewport};
use wgpu::RenderPass;

use crate::Error;

pub(crate) fn render(
    rp: &mut RenderPass,
    ta: &RwLock<TextAtlas>,
    tr: &RwLock<TextRenderer>,
    viewport: &RwLock<Viewport>
) -> Result<(), Error> {
    let ta = &ta.read().unwrap();
    let tr = tr.read().unwrap();
    let viewport = &viewport.read().unwrap();
    
    tr.render(ta, viewport, rp)?;

    Ok(())
}