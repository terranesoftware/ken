use std::sync::Arc;

use winit::{dpi::PhysicalPosition, window::Window};

use crate::{Error, page::Resources};

pub(crate) struct Editor {
    pub(crate) resources: Resources,
    pub(crate) window: Arc<Window>
}

impl Editor {
    pub(crate) fn moved(
        &self,
        position: PhysicalPosition<f64>
    ) {
        todo!()
    }

    pub(crate) fn render(&self) -> Result<(), Error> {
        todo!()
    }
}