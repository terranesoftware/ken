use std::sync::Arc;

use glyphon::{FontSystem, SwashCache, TextAtlas, TextRenderer, Viewport};
use wgpu::{Device, Queue};

use crate::{Error, screen::page::{Palette, landing::{Landing, Layout}}};

pub(super) fn layout(
    device: &Arc<Device>,
    fs: &mut FontSystem,
    landing: &Landing,
    layout: Layout,
    palette: Palette,
    queue: &Arc<Queue>,
    swc: &mut SwashCache,
    ta: &mut TextAtlas,
    tr: &mut TextRenderer,
    viewport: &Viewport,
) -> Result<(), Error> {
    let ink = palette.ink.text();

    let name = landing.name.read().unwrap();
    let new = landing.new.read().unwrap();
    let open = landing.open.read().unwrap();
    let re = landing.re.read().unwrap();
    let recent = landing.recent.read().unwrap();
    let tagline = landing.tagline.read().unwrap();

    tr.prepare(device, queue, fs, ta, viewport, [
        (name.1)(&name.0, layout, ink),
        (tagline.1)(&tagline.0, layout, ink)
    ].into_iter().chain(new.areas(layout, ink)).chain(open.areas(layout, ink)).chain([
        (recent.1)(&recent.0, layout, ink),
        (re.1)(&re.0, layout, ink)
    ]), swc)?;

    Ok(())
}