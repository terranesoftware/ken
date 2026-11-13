use wgpu::{CommandEncoderDescriptor, CurrentSurfaceTexture, Extent3d, Origin3d, TexelCopyTextureInfo, TextureAspect};
use winit::window::WindowId;

use crate::{App, app::window::Window};

impl App {
    pub(super) fn present(
        &mut self,
        window: WindowId
    ) {
        let window = self.windows.get(&window).unwrap();
        let (device, queue, sc, surface, texture) = match window {
            Window::Pending(_) => return,
            Window::Ready(page) => {
                let resources = page.resources();
                let gpu = &resources.gpu;

                (&gpu.device, &gpu.queue, &gpu.sc, &gpu.surface, &gpu.texture)
            }
        };
        
        // Get the current frame
        let frame = match surface.get_current_texture() {
            CurrentSurfaceTexture::Suboptimal(frame) |
            CurrentSurfaceTexture::Success(frame) => frame,
            _ => return
        };

        // Create the encoder using the device
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());
        
        // Copy the persistent texture over to the frame
        encoder.copy_texture_to_texture(TexelCopyTextureInfo {
            aspect: TextureAspect::All,
            mip_level: 0,
            origin: Origin3d::ZERO,
            texture
        }, TexelCopyTextureInfo {
            aspect: TextureAspect::All,
            mip_level: 0,
            origin: Origin3d::ZERO,
            texture: &frame.texture
        }, Extent3d {
            depth_or_array_layers: 1,
            height: sc.height,
            width: sc.width
        });
        
        // Submit the frame
        queue.submit([encoder.finish()]);
        queue.present(frame);
    }
}