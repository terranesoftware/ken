use std::sync::Arc;

use glyphon::{Color, FontSystem, Resolution, SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer, Viewport};
use wgpu::{Device, Queue, SurfaceConfiguration};
use winit::window::Window;

use crate::{Error, app::{area, buffers::Landing, utils::center}};

pub(crate) fn prepare(
    device: &Arc<Device>,
    fs: &mut FontSystem,
    landing: &mut Landing,
    queue: &Arc<Queue>,
    suc: &SurfaceConfiguration,
    swc: &mut SwashCache,
    ta: &mut TextAtlas,
    tr: &mut TextRenderer,
    viewport: &mut Viewport,
    window: &Window
) -> Result<(), Error> {
    landing.shape(fs);
    
    viewport.update(&queue, Resolution { height: suc.height, width: suc.width });

    let wd = window.inner_size();

    let top = (wd.height as f32 - 340.0) / 2.0;
    tr.prepare(&device, &queue, fs, ta, viewport, [
        area(&landing.name, center(&landing.name, wd.width as f32), top),
        area(&landing.tagline, center(&landing.name, wd.width as f32), top + 48.0),
        
        /* area(&landing.new, left, top),
        area(&landing.nc, left, top),

        area(&landing.open, left, top),
        area(&landing.oc, left, top),

        area(&landing.recent, left, top),
        area(&landing.re, left, top) */
    ], swc)?;

    Ok(())
}