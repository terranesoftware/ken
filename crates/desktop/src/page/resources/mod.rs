mod gpu;
use std::sync::Arc;

use gpu::Gpu;

mod text;
use text::Text;
use tokio::task::spawn_blocking;
use winit::window::Window;

use crate::{Error, app::resources::Gpu as AppGpu};

pub(crate) struct Resources {
    pub(crate) gpu: Gpu,
    pub(crate) text: Text
}

impl Resources {
    pub(super) async fn new(
        gpu: Arc<AppGpu>,
        window: Arc<Window>
    ) -> Result<Self, Error> {
        let device = gpu.device.clone();
        let queue = gpu.queue.clone();
        
        let moved = window.clone();
        let init = spawn_blocking(|| Gpu::init(gpu, moved));
        
        let (cache, viewport) = Text::init(&device);

        let (sc, surface) = init.await??;

        let tf = sc.format;

        let md = device.clone();
        let mq = queue.clone();
        let gpu = spawn_blocking(|| Gpu::complete(md, mq, sc, surface, window));
        let text = spawn_blocking(move || Text::complete(cache, device, queue, tf, viewport));

        let gpu = gpu.await?;
        let text = text.await?;
        
        Ok(Self {
            gpu,
            text
        })
    }
}