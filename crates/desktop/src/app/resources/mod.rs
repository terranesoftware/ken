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
    pub(super) fn new(
        gpu: Gpu,
        text: Text
    ) -> Self {
        Self {
            gpu: Arc::new(gpu),
            text: Arc::new(text)
        }
    }
    
    pub(super) async fn init() -> Result<(
        Result<Gpu, Error>,
        Text
    ), Error> {
        let gpu = spawn(Gpu::new(None));
        
        let text = spawn_blocking(Text::new);

        let gpu = gpu.await?;
        let text = text.await?;
        
        Ok((gpu, text))
    }
}