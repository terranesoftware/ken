mod gpu;
pub(crate) use gpu::Gpu;

mod text;
pub(crate) use text::Text;

use std::sync::Arc;

use tokio::{spawn, task::spawn_blocking};

use crate::Error;

pub(super) struct Resources {
    pub(super) gpu: Arc<Gpu>,
    pub(super) text: Arc<Text>
}

impl Resources {
    pub(super) async fn new() -> Result<Self, Error> {
        let gpu = spawn(Gpu::new(None));
        
        let text = spawn_blocking(Text::new);
        
        Ok(Self {
            gpu: Arc::new(gpu.await??),
            text: Arc::new(text.await?)
        })
    }
}