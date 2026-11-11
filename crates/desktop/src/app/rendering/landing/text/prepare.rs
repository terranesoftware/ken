use std::sync::Arc;

use glyphon::{Color, FontSystem, Resolution, SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer, Viewport};
use wgpu::{Device, Queue, SurfaceConfiguration};

use crate::{Error, app::buffers::Landing};

pub(crate) fn prepare(
    device: &Arc<Device>,
    fs: &mut FontSystem,
    landing: &mut Landing,
    queue: &Arc<Queue>,
    suc: &SurfaceConfiguration,
    swc: &mut SwashCache,
    ta: &mut TextAtlas,
    tr: &mut TextRenderer,
    viewport: &mut Viewport
) -> Result<(), Error> {
    landing.shape(fs);
    
    viewport.update(&queue, Resolution { height: suc.height, width: suc.width });

    tr.prepare(&device, &queue, fs, ta, viewport, [
        TextArea {
            bounds: TextBounds::default(),
            buffer: &landing.name,
            custom_glyphs: &[],
            default_color: Color::rgb(41, 40, 35),
            left: 40.0,
            scale: 1.0,
            top: 40.0
        }
    ], swc)?;

    Ok(())
}