mod background;
pub(super) use background::background;

mod landing;

use wgpu::{CommandEncoder, CommandEncoderDescriptor, CurrentSurfaceTexture, Extent3d, Origin3d, TexelCopyTextureInfo, TextureAspect, TextureView};

use crate::App;

impl App {
    pub(super) fn present(&mut self) {
        // We can only present with the GPU
        let Some(gpu) = &self.gpu else {
            return;
        };
        
        // Get everything we need to present from the GPU
        let (device, Some(sc), Some(surface), Some(texture)) = (&gpu.device, &gpu.sc, &gpu.surface, &gpu.texture) else {
            return;
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
        let queue = &gpu.queue;
        queue.submit([encoder.finish()]);
        queue.present(frame);
    }
    
    pub(super) fn render(
        &self,
        function: fn(&mut CommandEncoder, &TextureView)
    ) {
        // We can only render with the GPU
        let Some(gpu) = &self.gpu else {
            return;
        };

        // Get everything we need to render from the GPU
        let (device, Some(view)) = (&gpu.device, &gpu.view) else {
            return;
        };

        // Create the encoder using the device
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());
        
        // Whatever thing to render
        function(&mut encoder, view);

        // Submit the frame
        let queue = &gpu.queue;
        queue.submit([encoder.finish()]);
    }
}