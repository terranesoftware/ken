use wgpu::{CommandEncoderDescriptor, CurrentSurfaceTexture, TextureViewDescriptor};

use crate::{Gpu, app::{background, buffers::Landing}};

pub(crate) fn landing(
    gpu: &Gpu,
    landing: &mut Landing
) {
    let Some(surface) = &gpu.surface else {
        return;
    };
    
    let frame = match surface.get_current_texture() {
        CurrentSurfaceTexture::Suboptimal(frame) |
        CurrentSurfaceTexture::Success(frame) => frame,
        _ => return
    };
    
    let view = frame.texture.create_view(&TextureViewDescriptor::default());
    
    let device = &gpu.device;
    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());

    let rp = background(&mut encoder, &view);

    // rects()
    // text(landing)

    drop(rp);

    let queue = &gpu.queue;
    queue.submit([encoder.finish()]);
    queue.present(frame);
}