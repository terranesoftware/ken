use glyphon::{TextAtlas, TextRenderer, Viewport};
use wgpu::RenderPass;

use crate::Error;

pub(crate) fn render(
    rp: &mut RenderPass,
    ta: &TextAtlas,
    tr: &TextRenderer,
    viewport: &Viewport
) -> Result<(), Error> {
    tr.render(ta, viewport, rp)?;

    Ok(())
}