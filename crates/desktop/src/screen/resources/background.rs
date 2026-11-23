use wgpu::{Color, CommandEncoder, LoadOp, Operations, RenderPass, RenderPassColorAttachment, RenderPassDescriptor, StoreOp, TextureView};

use crate::screen::palette::Rgb;

pub(super) fn background<'encoder>(
    encoder: &'encoder mut CommandEncoder,
    view: &TextureView,
    paper: Rgb
) -> RenderPass<'encoder> {
    let [r, g, b, _] = paper.rect();

    encoder.begin_render_pass(&RenderPassDescriptor {
        color_attachments: &[
            Some(RenderPassColorAttachment {
                depth_slice: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: convert(r as f64),
                        g: convert(g as f64),
                        b: convert(b as f64),
                        a: 1.0
                    }),
                    store: StoreOp::Store
                },
                resolve_target: None,
                view: &view
            })
        ],
        label: Some("background"),
        ..Default::default()
    })
}

fn convert(channel: f64) -> f64 {
    if channel <= 0.04045 {
        channel / 12.92
    }
    else {
        ((channel + 0.055) / 1.055).powf(2.4)
    }
}