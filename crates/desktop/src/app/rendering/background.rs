use wgpu::{Color, CommandEncoder, LoadOp, Operations, RenderPass, RenderPassColorAttachment, RenderPassDescriptor, StoreOp, TextureView};

pub(crate) fn background<'a>(
    encoder: &'a mut CommandEncoder,
    view: &TextureView
) -> RenderPass<'a> {
    encoder.begin_render_pass(&RenderPassDescriptor {
        color_attachments: &[
            Some(RenderPassColorAttachment {
                depth_slice: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: 248.0 / 255.0,
                        g: 244.0 / 255.0,
                        b: 233.0 / 255.0,
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