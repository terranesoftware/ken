mod editor;
pub(super) use editor::Editor;

mod landing;
pub(super) use landing::Landing;

mod resources;
use resources::Resources;

pub(super) enum Page {
    Editor(Editor),
    Landing(Landing)
}

impl Page {
    pub(crate) fn resources(&self) -> &Resources {
        match self {
            Page::Editor(editor) => &editor.resources,
            Page::Landing(landing) => &landing.resources
        }
    }
}