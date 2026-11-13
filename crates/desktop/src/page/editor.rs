use winit::window::Window;

use crate::{Page, page::Resources};

pub(crate) struct Editor {
    pub(crate) resources: Resources,
    pub(crate) window: Window
}

impl Page {
    pub(crate) fn editor() -> Self {
        todo!()
    }
}