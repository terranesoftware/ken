mod buffers;
pub(super) use buffers::Buffers;

mod render;

use std::sync::{Arc, RwLock};

use carreau::Rect;
use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::{Error, screen::{Page, Resources, page::Destination}};

pub(crate) struct Editor {
    pub(crate) buffers: Arc<RwLock<Buffers>>,
    pub(crate) rects: Arc<RwLock<Vec<Rect>>>
}

impl Page {
    pub(crate) async fn editor(
        resources: &Arc<Resources>,
        size: PhysicalSize<u32>
    ) -> Result<Self, Error> {
        let editor = Editor {
            buffers: Arc::new(RwLock::new(Buffers::new())),
            rects: Arc::new(RwLock::new(Vec::new()))
        };
        editor.prepare(resources, size).await?;
        editor.render(resources)?;
        
        Ok(Self::Editor(editor))
    }
}

impl Editor {
    pub(super) fn clicked(
        &self,
        position: PhysicalPosition<f64>
    ) -> Option<Destination> {
        None
    }
    
    pub(super) fn moved(
        &self,
        position: PhysicalPosition<f64>,
        resources: &Resources
    ) {
        
    }
}