use carreau::Carreau;
use wgpu::{RenderPass, Texture};

pub(crate) fn render(
    carreau: &Carreau,
    rp: &mut RenderPass,
    texture: &Texture
) {
    carreau.render(rp, [texture.width() as f32, texture.height() as f32]);
}
