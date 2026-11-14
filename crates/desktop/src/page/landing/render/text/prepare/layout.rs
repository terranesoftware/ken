use std::sync::Arc;

use glyphon::{FontSystem, SwashCache, TextAtlas, TextRenderer, Viewport};
use wgpu::{Device, Queue};
use winit::dpi::PhysicalSize;

use crate::{Error, page::{Buffers, landing::render::text::{area, center, width}}};

pub(super) fn layout(
    buffers: &Buffers,
    device: &Arc<Device>,
    fs: &mut FontSystem,
    queue: &Arc<Queue>,
    swc: &mut SwashCache,
    ta: &mut TextAtlas,
    tr: &mut TextRenderer,
    viewport: &Viewport,
    wd: PhysicalSize<u32>
) -> Result<(), Error> {
    let h = wd.height as f32;
    let w = wd.width as f32;
    
    let cw = w - 2000.0;
    let left = (w - cw) / 2.0;
    let right = cw + left;
    
    let top = (h - 1000.0) / 2.0;
    tr.prepare(device, queue, fs, ta, viewport, [
        area(&buffers.name, center(&buffers.name, w), top),
        area(&buffers.tagline, center(&buffers.tagline, w), top + 175.0),
        
        area(&buffers.new, left, top + 350.0),
        area(&buffers.nc, right - width(&buffers.nc), top + 350.0),
        
        area(&buffers.open, left, top + 500.0),
        area(&buffers.oc, right - width(&buffers.nc), top + 500.0),
        
        area(&buffers.recent, left, top + 750.0),
        area(&buffers.re, left, top + 850.0)
    ], swc)?;

    Ok(())
}