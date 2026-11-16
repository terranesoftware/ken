use std::sync::Arc;

use glyphon::{FontSystem, SwashCache, TextAtlas, TextRenderer, Viewport};
use wgpu::{Device, Queue};

use crate::{Error, page::{Buffers, landing::render::{LAYOUT, text::{area, center, width}}}};

pub(super) fn layout(
    buffers: &Buffers,
    device: &Arc<Device>,
    fs: &mut FontSystem,
    queue: &Arc<Queue>,
    swc: &mut SwashCache,
    ta: &mut TextAtlas,
    tr: &mut TextRenderer,
    viewport: &Viewport,
) -> Result<(), Error> {
    let (_, left, right, top, w) = *LAYOUT.get().unwrap();
    
    tr.prepare(device, queue, fs, ta, viewport, [
        area(&buffers.name, center(&buffers.name, w), top),
        area(&buffers.tagline, center(&buffers.tagline, w), top + 175.0),
        
        area(&buffers.ni, left + 25.0, top + 350.0),
        area(&buffers.new, left + 100.0, top + 350.0),
        area(&buffers.nc, right - width(&buffers.nc) - 25.0, top + 350.0),
        
        area(&buffers.oi, left + 25.0, top + 500.0),
        area(&buffers.open, left + 100.0, top + 500.0),
        area(&buffers.oc, right - width(&buffers.nc) - 25.0, top + 500.0),
        
        area(&buffers.recent, left, top + 750.0),
        area(&buffers.re, left, top + 850.0)
    ], swc)?;

    Ok(())
}