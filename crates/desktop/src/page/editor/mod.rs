mod buffers;
pub(super) use buffers::Buffers;
use wgpu::Surface;

mod render;

use std::sync::{Arc, RwLock};

use carreau::Rect;
use winit::{dpi::PhysicalPosition, window::Window};

use crate::{Error, Page, page::Resources, resources::{Gpu, Text}};

pub(crate) struct Editor {
    pub(crate) buffers: Arc<RwLock<Buffers>>,
    pub(crate) rects: Arc<RwLock<Vec<Rect>>>,
    pub(crate) resources: Resources,
    pub(crate) window: Arc<Window>
}

impl Page {
    pub(crate) async fn editor(
        gpu: Arc<Gpu>,
        surface: Surface<'static>,
        text: Arc<Text>,
        window: Arc<Window>
    ) -> Result<Self, Error> {
        let resources = Resources::new(gpu, surface, text, window.clone()).await?;
        
        let editor = Editor {
            buffers: Arc::new(RwLock::new(Buffers::new())),
            rects: Arc::new(RwLock::new(Vec::new())),
            resources,
            window
        };
        editor.prepare().await?;
        editor.render()?;
        
        Ok(Self::Editor(editor))
    }
}

impl Editor {
    pub(crate) fn moved(
        &self,
        position: PhysicalPosition<f64>
    ) {
        todo!()
    }
}