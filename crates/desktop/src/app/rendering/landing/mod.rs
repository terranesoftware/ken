mod text;

use glyphon::{FontSystem, SwashCache};

use wgpu::{CommandEncoderDescriptor, CurrentSurfaceTexture, TextureViewDescriptor};
use winit::window::Window;

use crate::{Error, Gpu, app::{background, buffers::Landing}};

pub(crate) fn landing(
    fs: &mut FontSystem,
    gpu: &mut Gpu,
    landing: &mut Landing,
    swc: &mut SwashCache,
    window: &Window
) -> Result<(), Error> {
    let (Some(suc), Some(text)) = (gpu.sc.as_ref(), gpu.text.as_mut()) else {
        return Ok(());
    };

    let (Some(ta), Some(tr), viewport) = (text.ta.as_mut(), text.tr.as_mut(), &mut text.viewport) else {
        return Ok(());
    };
    
    text::prepare(&gpu.device, fs, landing, &gpu.queue, suc, swc, ta, tr, viewport, window)?;
    
    let Some(surface) = &gpu.surface else {
        return Ok(());
    };
    
    let frame = match surface.get_current_texture() {
        CurrentSurfaceTexture::Suboptimal(frame) |
        CurrentSurfaceTexture::Success(frame) => frame,
        _ => return Ok(())
    };
    
    let view = frame.texture.create_view(&TextureViewDescriptor::default());
    
    let device = &gpu.device;
    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());

    let mut rp = background(&mut encoder, &view);

    // rects()
    text::render(&mut rp, ta, tr, viewport);

    drop(rp);

    let queue = &gpu.queue;
    queue.submit([encoder.finish()]);
    queue.present(frame);

    Ok(())
}