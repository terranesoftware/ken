mod buffers;
use buffers::Buffers;

mod gpu;
pub(super) use gpu::Gpu;

mod handler;

mod rendering;
use rendering::background;

use std::sync::Arc;

use glyphon::{FontSystem, SwashCache};
use tokio::{spawn, task::spawn_blocking};
use wgpu::{RequestAdapterOptions, SurfaceTargetUnsafe};
use winit::{event_loop::EventLoopProxy, window::Window};

use crate::{Error, UserEvent};

pub(super) struct App {
    buffers: Buffers,
    elp: Arc<EventLoopProxy<UserEvent>>,
    fs: Option<FontSystem>,
    gpu: Option<Gpu>,
    sc: SwashCache,
    window: Option<Window>
}

impl App {
    pub(super) fn new(elp: EventLoopProxy<UserEvent>) -> Self {
        // Wrap it in an Arc because on some platforms, cloning it is expensive
        let elp = Arc::new(elp);
        
        // Asynchronously initialize the GPU since its expensive
        let moved = elp.clone();
        spawn(async move {
            let result = Gpu::new(None).await;
            _ = moved.send_event(UserEvent::Gpu(result));
        });

        let moved = elp.clone();
        spawn_blocking(move || {
            let fs = FontSystem::new();
            _ = moved.send_event(UserEvent::FontSystem(fs));
        });

        Self {
            buffers: Buffers::new(),
            elp,
            fs: None,
            gpu: None,
            sc: SwashCache::new(),
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
                _ = moved_elp.send_event(UserEvent::Gpu(result));
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