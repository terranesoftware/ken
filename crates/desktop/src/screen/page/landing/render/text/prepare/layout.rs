use std::sync::Arc;

use glyphon::{FontSystem, SwashCache, TextAtlas, TextRenderer, Viewport};
use wgpu::{Device, Queue};

use crate::{Error, screen::page::landing::{Buffers, Layout, render::text::{area, center, width}}};

pub(super) fn layout(
    buffers: &Buffers,
    device: &Arc<Device>,
    fs: &mut FontSystem,
    layout: Layout,
    queue: &Arc<Queue>,
    swc: &mut SwashCache,
    ta: &mut TextAtlas,
    tr: &mut TextRenderer,
    viewport: &Viewport,
) -> Result<(), Error> {
    let Layout { left, right, s, top, w, .. } = layout;
    
    tr.prepare(device, queue, fs, ta, viewport, [
        area(&buffers.name, center(&buffers.name, w), top, s),
        area(&buffers.tagline, center(&buffers.tagline, w), top + 87.5, s),
        
        area(&buffers.ni, left + 12.5, top + 175.0, s),
        area(&buffers.new, left + 50.0, top + 175.0, s),
        area(&buffers.nc, right - width(&buffers.nc) - 12.5, top + 175.0, s),
        
        area(&buffers.oi, left + 12.5, top + 250.0, s),
        area(&buffers.open, left + 50.0, top + 250.0, s),
        area(&buffers.oc, right - width(&buffers.nc) - 12.5, top + 250.0, s),
        
        area(&buffers.recent, left, top + 375.0, s),
        area(&buffers.re, left, top + 425.0, s)
    ], swc)?;

    Ok(())
}