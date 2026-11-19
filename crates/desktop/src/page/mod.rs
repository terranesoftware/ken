mod editor;
pub(super) use editor::Editor;

mod landing;
pub(super) use landing::Landing;

mod resources;
use resources::Resources;

use std::sync::Arc;

use winit::{dpi::PhysicalPosition, window::Window};

use crate::Error;

pub(super) enum Page {
    Editor(Editor),
    Landing(Landing)
}

impl Page {
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
            Self::Landing(landing) => landing.render()
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