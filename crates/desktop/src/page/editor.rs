use std::sync::Arc;

use winit::window::Window;

use crate::{Page, page::Resources};

pub(crate) struct Editor {
    pub(crate) resources: Resources,
    pub(crate) window: Arc<Window>
}

impl Page {
    pub(crate) fn editor() -> Self {
        todo!()
    }
}