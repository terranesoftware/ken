mod buffers;
use buffers::Buffers;

pub(super) mod gpu;
pub(crate) use gpu::Gpu;

mod handler;

mod rendering;

mod utils;
use utils::area;

use std::sync::Arc;

use glyphon::{FontSystem, SwashCache, TextAtlas, TextRenderer};
use tokio::{spawn, task::spawn_blocking};
use wgpu::{Extent3d, MultisampleState, SurfaceTargetUnsafe, TextureDescriptor, TextureDimension, TextureUsages, TextureViewDescriptor};
use winit::{event_loop::EventLoopProxy, window::Window};

use crate::{Error, UserEvent};

pub(super) struct App {
    buffers: Buffers,
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
            let result = Gpu::new(moved.clone(), None).await;
            _ = moved.send_event(UserEvent::Gpu(result));
        });

        let moved = elp.clone();
        spawn_blocking(move || {
            let mut fs = FontSystem::new();
            
            let database = fs.db_mut();
            database.load_font_data(include_bytes!("../../fonts/BricolageGrotesque-Bold.ttf").to_vec());
            database.load_font_data(include_bytes!("../../fonts/Fraunces_72pt-Bold.ttf").to_vec());
            database.load_font_data(include_bytes!("../../fonts/InstrumentSerif-Regular.ttf").to_vec());
            database.load_font_data(include_bytes!("../../fonts/Righteous-Regular.ttf").to_vec());
            
            _ = moved.send_event(UserEvent::FontSystem(fs));
        });

        Self {
            buffers: Buffers::new(),
            fs: None,
            gpu: None,
            sc: SwashCache::new(),
            window: None
        }
    }

    fn gpu(&mut self) -> Result<(), Error> {
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

    fn text(&mut self) {
        // This check exists because its technically possible that the Text thread completes before the GPU field gets filled
        let Some(gpu) = &mut self.gpu else {
            return;
        };

        // Make sure both the SurfaceConfiguration and the Text are created, otherwise wait for the second call
        let (device, queue, Some(sc), Some(text)) = (&gpu.device, &gpu.queue, &gpu.sc, &mut gpu.text) else {
            return;
        };

        let mut ta = TextAtlas::new(device, queue, &text.cache, sc.format);

        let tr = TextRenderer::new(&mut ta, device, MultisampleState::default(), None);

        text.ta = Some(ta);
        text.tr = Some(tr);
    }
}