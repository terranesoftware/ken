mod buffers;
pub(super) use buffers::Buffers;

mod layout;
use layout::Layout;

mod render;

use std::sync::{Arc, RwLock};

use carreau::Rect;
use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::{Error, screen::{Page, Resources, page::{Destination, PageMethods}}};

pub(crate) struct Editor {
    pub(crate) buffers: Arc<RwLock<Buffers>>,
    pub(crate) rects: Arc<RwLock<Vec<Rect>>>
}

impl Page {
    pub(crate) async fn editor(
        resources: &Arc<Resources>,
        scale: f32,
        size: PhysicalSize<u32>
    ) -> Result<Self, Error> {
        let editor = Editor {
            buffers: Arc::new(RwLock::new(Buffers::new())),
            rects: Arc::new(RwLock::new(Vec::new()))
        };
        editor.prepare(Layout::new(scale, size), resources).await?;
        editor.render(resources)?;
        
        Ok(Self::Editor(editor))
    }
}

impl PageMethods for Editor {
    fn clicked(
        &self,
        position: PhysicalPosition<f64>
    ) -> Option<Destination> {
        None
    }
    
    fn moved(
        &self,
        position: PhysicalPosition<f64>,
        resources: &Resources
    ) {
        
    }

    fn render(
        &self,
        resources: &Arc<Resources>
    ) -> Result<(), Error> {
        Ok(())
    }
}