use wgpu::{CommandEncoderDescriptor, CurrentSurfaceTexture, Extent3d, Origin3d, TexelCopyTextureInfo, TextureAspect};
use winit::window::WindowId;

use crate::App;

impl App {
    pub(super) fn present(
        &mut self,
        window: WindowId
    ) {
        let Some(screen) = self.screens.get(&window) else {
            return;
        };
        
        let (device, queue, sc, surface, texture) = {
            let gpu = &screen.resources.gpu;
            let (surface, sc) = &screen.surface;
            
            (&gpu.device, &gpu.queue, sc, surface, screen.resources.tv.texture())
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