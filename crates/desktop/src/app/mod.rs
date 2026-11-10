mod gpu;
use gpu::Gpu;

mod handler;

use winit::window::Window;

pub(super) struct App {
    gpu: Gpu,
    window: Option<Window>
}

impl App {
    pub(super) async fn new() -> Self {
        let gpu = Gpu::new().await;
        
        Self {
            gpu,
            window: None
        }
    }
}