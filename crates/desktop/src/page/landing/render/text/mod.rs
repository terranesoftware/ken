mod utils;
use utils::{area, center};

use std::sync::{Arc, RwLock};

use glyphon::{Resolution, TextAtlas, TextRenderer, Viewport};
use wgpu::RenderPass;
use winit::window::Window;

use crate::{Error, page::{Buffers, resources::{Gpu, Text}}};

pub(crate) fn prepare(
    buffers: Arc<RwLock<Buffers>>,
    gpu: Arc<Gpu>,
    text: Arc<Text>,
    window: Arc<Window>
) -> Result<(), Error> {
    let buffers = &mut buffers.write().unwrap();
    let fs = &mut text.fs.write().unwrap();
    buffers.shape(fs);
    
    let queue = &gpu.queue;
    let suc = &gpu.sc;
    let viewport = &text.viewport;
    {
        let viewport = &mut viewport.write().unwrap();
        viewport.update(queue, Resolution {
            height: suc.height,
            width: suc.width
        });
    }
    
    let device = &gpu.device;
    let queue = &gpu.queue;
    let swc = &mut text.sc.write().unwrap();
    let ta = &mut text.ta.write().unwrap();
    let tr = &mut text.tr.write().unwrap();
    let viewport = &viewport.read().unwrap();
    let wd = window.inner_size();

    let top = (wd.height as f32 - 340.0) / 2.0;
    tr.prepare(device, queue, fs, ta, viewport, [
        area(&buffers.name, center(&buffers.name, wd.width as f32), top),
        area(&buffers.tagline, center(&buffers.name, wd.width as f32), top + 48.0),
        
        /* area(&buffers.new, left, top),
        area(&buffers.nc, left, top),

        area(&buffers.open, left, top),
        area(&buffers.oc, left, top),

        area(&buffers.recent, left, top),
        area(&buffers.re, left, top) */
    ], swc)?;

    Ok(())
}

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