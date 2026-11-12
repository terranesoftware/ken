use std::sync::Arc;

use wgpu::{Extent3d, SurfaceTargetUnsafe, TextureDescriptor, TextureDimension, TextureUsages, TextureViewDescriptor};

use crate::{App, Error};

impl App {
    pub(crate) fn gpu(&mut self) -> Result<(), Error> {
        // Check if both the GPU and the Window are created, otherwise, wait for the second call
        let (Some(gpu), Some(window)) = (&mut self.gpu, &self.window) else {
            return Ok(());
        };
        
        let instance = &gpu.instance;
        
        // Create an unsafe target (wrapping this in an Arc is OD) and make a surface out of it
        let target = unsafe { SurfaceTargetUnsafe::from_display_and_window(window, window)? };
        gpu.surface = unsafe { Some(Arc::new(instance.create_surface_unsafe(target)?)) };
        
        let adapter = &gpu.adapter;
        let surface = gpu.surface.as_ref().unwrap();
        
        // TODO:
        // If the surface isn't supported, retry, guaranteeing it'll work (or if no GPUs are compatible, it'll just error)
        if !adapter.is_surface_supported(surface) {
            todo!();
        }
        
        // Create a default config
        let size = window.inner_size();
        gpu.sc = surface.get_default_config(adapter, size.width, size.height);
        
        let device = &gpu.device;
        let sc = gpu.sc.as_mut().unwrap();
        
        // Get the maximum height of any monitor and maximum width of any monitor
        let (height, width) = window.available_monitors().fold(
            (0, 0),
            |(height, width), monitor| (height.max(monitor.size().height), width.max(monitor.size().width))
        );
        
        // Create a persistent texture for efficiency (retained rendering)
        sc.usage |= TextureUsages::COPY_DST;
        let texture = device.create_texture(&TextureDescriptor {
            dimension: TextureDimension::D2,
            format: sc.format,
            label: None,
            mip_level_count: 1,
            sample_count: 1,
            size: Extent3d {
                depth_or_array_layers: 1,
                height,
                width,
            },
            usage: TextureUsages::COPY_SRC | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[]
        });
        let view = texture.create_view(&TextureViewDescriptor::default());
        
        gpu.texture = Some(texture);
        gpu.view = Some(view);
        
        let device = &gpu.device;
        let sc = gpu.sc.as_ref().unwrap();
        
        // Configure the surface
        surface.configure(device, sc);
        
        // Surface is configured, we're ready to do text
        self.text();
        
        Ok(())
    }
}