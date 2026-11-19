mod editor;
pub(super) use editor::Editor;

mod landing;
pub(super) use landing::Landing;

mod resources;
use resources::Resources;

use std::sync::{Arc, RwLock};

use wgpu::Surface;
use winit::{dpi::PhysicalPosition, window::Window};

use crate::{Error, resources::{Gpu, Text}, page::landing::Buffers};

pub(super) enum Page {
    Editor(Editor),
    Landing(Landing)
}

impl Page {
    pub(crate) fn editor() {
        todo!();
    }
    
    pub(crate) async fn landing(
        gpu: Arc<Gpu>,
        surface: Surface<'static>,
        text: Arc<Text>,
        window: Arc<Window>
    ) -> Result<Self, Error> {
        let resources = Resources::new(gpu, surface, text, window.clone()).await?;

        let landing = Landing {
            buffers: Arc::new(RwLock::new(Buffers::new())),
            rects: Arc::new(RwLock::new(Vec::new())),
            resources,
            window
        };
        landing.prepare().await?;
        landing.render().await?;
        
        Ok(Self::Landing(landing))
    }

    pub(crate) fn moved(
        &self,
        position: PhysicalPosition<f64>
    ) {
        match self {
            Self::Editor(editor) => editor.moved(position),
            Self::Landing(landing) => landing.moved(position)
        }
    }

    pub(crate) async fn render(&self) -> Result<(), Error> {
        match self {
            Self::Editor(editor) => editor.render(),
            Self::Landing(landing) => landing.render().await
        }
    }
    
    pub(crate) fn resources(&self) -> &Resources {
        match self {
            Self::Editor(editor) => &editor.resources,
            Self::Landing(landing) => &landing.resources
        }
    }

    pub(crate) fn window(&self) -> Arc<Window> {
        match self {
            Self::Editor(editor) => editor.window.clone(),
            Self::Landing(landing) => landing.window.clone()
        }
    }
}