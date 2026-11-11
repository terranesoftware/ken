mod background;
use background::background;

mod landing;
use landing::landing;

use wgpu::{CommandEncoderDescriptor, CurrentSurfaceTexture, TextureViewDescriptor};

use crate::App;

pub(super) fn render(app: &mut App) {
    let Some(gpu) = &app.gpu else {
        return;
    };
    
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
    
    drop(rp);

    let queue = &gpu.queue;
    queue.submit([encoder.finish()]);
    queue.present(frame);
}