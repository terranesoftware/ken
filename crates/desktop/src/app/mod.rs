mod buffers;
use buffers::Buffers;

pub(super) mod gpu;
pub(crate) use gpu::Gpu;

mod handler;

mod rendering;

mod utils;
use utils::area;

use std::sync::Arc;

use glyphon::{FontSystem, SwashCache};
use tokio::{spawn, task::spawn_blocking};
use winit::{event_loop::EventLoopProxy, window::Window};

use crate::UserEvent;

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
}