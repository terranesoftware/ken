mod gpu;
pub(super) use gpu::Gpu;

mod text;
pub(super) use text::Text;

use std::sync::Arc;

use tokio::task::spawn_blocking;
use wgpu::Surface;
use winit::window::Window;

use crate::{Error, app::resources::{Gpu as AppGpu, Text as AppText}};

pub(crate) struct Resources {
    pub(crate) gpu: Arc<Gpu>,
    pub(crate) text: Arc<Text>
}

impl Resources {
    pub(super) async fn new(
        gpu: Arc<AppGpu>,
        surface: Surface<'static>,
        text: Arc<AppText>,
        window: Arc<Window>
    ) -> Result<Self, Error> {
        let device = gpu.device.clone();
        let queue = gpu.queue.clone();

        let moved = window.clone();
        let init = spawn_blocking(|| Gpu::init(gpu, surface, moved));
        
        let (cache, viewport) = Text::init(&device);

        let (sc, surface) = init.await??;

        let tf = sc.format;

        let md = device.clone();
        let mq = queue.clone();
        let gpu = spawn_blocking(|| Gpu::complete(md, mq, sc, surface, window));

        let fs = text.fs.clone();
        let sc = text.sc.clone();
        let text = spawn_blocking(move || Text::complete(cache, fs, device, queue, sc, tf, viewport));

        let gpu = Arc::new(gpu.await?);
        let text = Arc::new(text.await?);
        
        Ok(Self {
            gpu,
            text
        })
    }
}