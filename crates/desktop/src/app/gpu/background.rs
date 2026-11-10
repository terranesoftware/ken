use wgpu::{Color, CommandEncoderDescriptor, CurrentSurfaceTexture, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor, StoreOp, TextureViewDescriptor};

use crate::Gpu;

impl Gpu {
    pub(crate) fn background(&self) {
        let Some(surface) = &self.surface else {
            return;
        };

        let frame = match surface.get_current_texture() {
            CurrentSurfaceTexture::Suboptimal(frame) |
            CurrentSurfaceTexture::Success(frame) => frame,
            _ => return
        };

        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor::default());

        _ = encoder.begin_render_pass(&RenderPassDescriptor {
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
        });

        self.queue.submit([encoder.finish()]);
        self.queue.present(frame);
    }
}