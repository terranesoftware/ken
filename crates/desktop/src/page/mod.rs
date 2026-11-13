mod editor;
pub(super) use editor::Editor;

mod landing;
pub(super) use landing::Landing;

mod resources;
use resources::Resources;
use winit::window::Window;

use std::sync::Arc;

use crate::{Error, app::resources::Gpu, page::landing::Buffers};

pub(super) enum Page {
    Editor(Editor),
    Landing(Landing)
}

impl Page {
    pub(crate) async fn landing(
        gpu: Arc<Gpu>,
        window: Window
    ) -> Result<Self, Error> {
        let window = Arc::new(window);
                
        let resources = Resources::new(gpu, window.clone()).await?;
        
        Ok(Self::Landing(Landing {
            buffers: Buffers::new(),
            resources,
            window: Arc::try_unwrap(window).unwrap()
        }))
    }
    
    pub(crate) fn resources(&self) -> &Resources {
        match self {
            Page::Editor(editor) => &editor.resources,
            Page::Landing(landing) => &landing.resources
        }
    }
}