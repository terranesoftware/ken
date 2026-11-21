use wgpu::{Color, CommandEncoder, LoadOp, Operations, RenderPass, RenderPassColorAttachment, RenderPassDescriptor, StoreOp, TextureView};

fn convert(channel: f64) -> f64 {
    if channel <= 0.04045 {
        channel / 12.92
    }
    else {
        ((channel + 0.055) / 1.055).powf(2.4)
    }
}

pub(super) fn background<'encoder>(
    encoder: &'encoder mut CommandEncoder,
    view: &TextureView
) -> RenderPass<'encoder> {
    encoder.begin_render_pass(&RenderPassDescriptor {
        color_attachments: &[
            Some(RenderPassColorAttachment {
                depth_slice: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: convert(248.0 / 255.0),
                        g: convert(244.0 / 255.0),
                        b: convert(233.0 / 255.0),
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