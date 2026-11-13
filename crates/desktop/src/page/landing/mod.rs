mod buffers;
use std::sync::Arc;

use buffers::Buffers;

use winit::window::Window;

use crate::{Error, Page, app::resources::Gpu, page::Resources};

pub(crate) struct Landing {
    pub(crate) buffers: Buffers,
    pub(crate) resources: Resources,
    pub(crate) window: Window
}

impl Landing {
    async fn new(
        gpu: Arc<Gpu>,
        window: Window
    ) -> Result<Self, Error> {
        let window = Arc::new(window);
        
        let resources = Resources::new(gpu, window.clone()).await?;

        Ok(Self {
            buffers: Buffers::new(),
            resources,
            window: Arc::try_unwrap(window).unwrap()
        })
    }
}

impl Page {
    pub(crate) async fn landing(
        gpu: Arc<Gpu>,
        window: Window
    ) -> Result<Self, Error> {
        Ok(Self::Landing(Landing::new(gpu, window).await?))
    }
}