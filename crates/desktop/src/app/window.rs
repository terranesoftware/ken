use winit::window::Window as WinitWindow;

use crate::Page;

pub(crate) enum Window {
    Pending(WinitWindow),
    Ready(Page)
}