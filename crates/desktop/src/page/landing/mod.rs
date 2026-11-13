mod buffers;
pub(super) use buffers::Buffers;

use winit::window::Window;

use crate::page::resources::Resources;

pub(crate) struct Landing {
    pub(crate) buffers: Buffers,
    pub(crate) resources: Resources,
    pub(crate) window: Window
}