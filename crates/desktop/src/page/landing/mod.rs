mod buffers;
use std::sync::{Arc, RwLock};

pub(super) use buffers::Buffers;

mod render;

use winit::{dpi::PhysicalPosition, window::Window};

use crate::page::resources::Resources;

pub(crate) struct Landing {
    pub(crate) buffers: Arc<RwLock<Buffers>>,
    pub(crate) resources: Resources,
    pub(crate) window: Arc<Window>
}

impl Landing {
    pub(super) fn moved(
        &self,
        position: PhysicalPosition<f64>
    ) {
        todo!();
    }
}