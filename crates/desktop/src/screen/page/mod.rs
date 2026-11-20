mod destination;
use std::sync::Arc;

pub(crate) use destination::Destination;

mod editor;
pub(super) use editor::Editor;

mod landing;
pub(super) use landing::Landing;

use winit::dpi::PhysicalPosition;

use crate::{Error, screen::Resources};

pub(crate) enum Page {
    Editor(Editor),
    Landing(Landing)
}

impl Page {
    pub(crate) fn clicked(
        &self,
        position: PhysicalPosition<f64>
    ) -> Option<Destination> {
        match self {
            Self::Editor(editor) => editor.clicked(position),
            Self::Landing(landing) => landing.clicked(position)
        }
    }
    
    pub(crate) fn moved(
        &self,
        position: PhysicalPosition<f64>,
        resources: &Resources
    ) {
        match self {
            Self::Editor(editor) => editor.moved(position, resources),
            Self::Landing(landing) => landing.moved(position, resources)
        }
    }

    pub(crate) async fn render(
        &self,
        resources: &Arc<Resources>
    ) -> Result<(), Error> {
        match self {
            Self::Editor(editor) => editor.render(resources),
            Self::Landing(landing) => landing.render(resources)
        }
    }
}