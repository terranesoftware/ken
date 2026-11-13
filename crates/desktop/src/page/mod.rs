mod editor;
pub(super) use editor::Editor;

mod landing;
pub(super) use landing::Landing;

mod resources;
use resources::Resources;
use wgpu::Surface;
use winit::window::Window;

use std::sync::{Arc, RwLock};

use crate::{Error, app::resources::{Gpu, Text}, page::landing::Buffers};

pub(super) enum Page {
    Editor(Editor),
    Landing(Landing)
}

impl Page {
    pub(crate) async fn landing(
        gpu: Arc<Gpu>,
        surface: Surface<'static>,
        text: Arc<Text>,
        window: Arc<Window>
    ) -> Result<Self, Error> {
        let resources = Resources::new(gpu, surface, text, window.clone()).await?;

        let landing = Landing {
            buffers: Arc::new(RwLock::new(Buffers::new())),
            resources,
            window
        };
        landing.render().await?;
        
        Ok(Self::Landing(landing))
    }
    
    pub(crate) fn resources(&self) -> &Resources {
        match self {
            Page::Editor(editor) => &editor.resources,
            Page::Landing(landing) => &landing.resources
        }
    }

    pub(crate) fn window(&self) -> Arc<Window> {
        match self {
            Page::Editor(editor) => editor.window.clone(),
            Page::Landing(landing) => landing.window.clone()
        }
    }
}