use wgpu::{Color, CommandEncoder, LoadOp, Operations, RenderPass, RenderPassColorAttachment, RenderPassDescriptor, StoreOp, TextureView};

pub(crate) fn background<'encoder>(
    encoder: &'encoder mut CommandEncoder,
    view: &TextureView
) -> RenderPass<'encoder> {
    encoder.begin_render_pass(&RenderPassDescriptor {
        color_attachments: &[
            Some(RenderPassColorAttachment {
                depth_slice: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: 242.0 / 255.0,
                        g: 234.0 / 255.0,
                        b: 216.0 / 255.0,
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