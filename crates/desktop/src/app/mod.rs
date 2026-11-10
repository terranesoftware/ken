mod gpu;
use std::sync::Arc;

pub(super) use gpu::Gpu;

mod handler;

use tokio::spawn;
use wgpu::{RequestAdapterOptions, SurfaceTargetUnsafe};
use winit::{event_loop::EventLoopProxy, window::Window};

use crate::Error;

pub(super) struct App {
    elp: Arc<EventLoopProxy<Result<Gpu, Error>>>,
    gpu: Option<Gpu>,
    window: Option<Window>
}

impl App {
    pub(super) fn new(elp: EventLoopProxy<Result<Gpu, Error>>) -> Self {
        // Wrap it in an Arc because on some platforms, cloning it is expensive
        let elp = Arc::new(elp);
        
        // Asynchronously initialize the GPU since its expensive
        let moved = elp.clone();
        spawn(async move {
            let result = Gpu::new(None).await;
            _ = moved.clone().send_event(result);
        });
        
        Self {
            elp,
            gpu: None,
            window: None
        }
    }

    fn complete(&mut self) -> Result<(), Error> {
        // Check if both the GPU and the Window are created, otherwise, wait for the second call
        let (elp, Some(gpu), Some(window)) = (&self.elp, &mut self.gpu, &self.window) else {
            return Ok(());
        };

        let instance = &gpu.instance;
        
        // Create an unsafe target (wrapping this in an Arc is OD) and make a surface out of it
        let target = unsafe { SurfaceTargetUnsafe::from_display_and_window(window, window)? };
        gpu.surface = unsafe { Some(Arc::new(instance.create_surface_unsafe(target)?)) };

        let adapter = &gpu.adapter;
        let surface = gpu.surface.as_ref().unwrap();
        
        // If the surface isn't supported, retry, guaranteeing it'll work (or if no GPUs are compatible, it'll just error)
        if !adapter.is_surface_supported(surface) {
            let moved_elp = elp.clone();
            let moved_surface = surface.clone();
            spawn(async move {
                let result = Gpu::new(Some(RequestAdapterOptions {
                    compatible_surface: Some(&moved_surface),
                    ..Default::default()
                })).await;
                _ = moved_elp.send_event(result);
            });
        }

        // Create a default config
        let size = window.inner_size();
        gpu.sc = surface.get_default_config(adapter, size.width, size.height);

        let device = &gpu.device;
        let sc = gpu.sc.as_ref().unwrap();

        // Configure the surface
        surface.configure(device, sc);

        Ok(())
    }
}