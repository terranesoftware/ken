use std::sync::Arc;

use glyphon::{FontSystem, SwashCache, TextAtlas, TextRenderer, Viewport};
use wgpu::{Device, Queue};

use crate::{Error, screen::page::editor::{Buffers, Layout, render::text::area}};

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
    let Layout { s, .. } = layout;
    
    tr.prepare(device, queue, fs, ta, viewport, [] as [glyphon::TextArea; 0], swc)?;

    Ok(())
}