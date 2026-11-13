use std::sync::Arc;

use wgpu::{Device, Extent3d, Queue, Surface, SurfaceConfiguration, SurfaceTargetUnsafe, Texture, TextureDescriptor, TextureDimension, TextureUsages, TextureView, TextureViewDescriptor};
use winit::window::Window;

use crate::{Error, app::resources::Gpu as AppGpu};

pub(crate) struct Gpu {
    pub(crate) device: Arc<Device>,
    pub(crate) queue: Arc<Queue>,
    pub(crate) sc: SurfaceConfiguration,
    pub(crate) surface: Surface<'static>,
    pub(crate) texture: Texture,
    pub(crate) view: TextureView
}

impl Gpu {
    pub(super) fn init(
        gpu: Arc<AppGpu>,
        window: Arc<Window>
    ) -> Result<(
        SurfaceConfiguration,
        Surface<'static>
    ), Error> {
        let instance = &gpu.instance;
        
        // Create an unsafe target (wrapping this in an Arc is OD) and make a surface out of it
        let target = unsafe { SurfaceTargetUnsafe::from_display_and_window(&window, &window)? };
        let surface = unsafe { instance.create_surface_unsafe(target)? };
        
        let adapter = &gpu.adapter;
        
        // TODO:
        // If the surface isn't supported, retry, guaranteeing it'll work (or if no GPUs are compatible, it'll just error)
        if !adapter.is_surface_supported(&surface) {
            todo!();
        }
        
        // Create a default config
        let size = window.inner_size();
        let sc = surface.get_default_config(adapter, size.width, size.height).unwrap();
        
        let device = &gpu.device;
        // Configure the surface
        surface.configure(device, &sc);

        Ok((sc, surface))
    }

    pub(super) fn complete(
        device: Arc<Device>,
        queue: Arc<Queue>,
        mut sc: SurfaceConfiguration,
        surface: Surface<'static>,
        window: Arc<Window>
    ) -> Self {
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
        
        Self {
            device: device.clone(),
            queue: queue.clone(),
            sc,
            surface,
            texture,
            view
        }
    }
}