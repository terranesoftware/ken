use winit::window::WindowId;

use crate::{Error, Page};

pub(super) enum UserEvent {
    Render(WindowId, Result<(), Error>),
    Window(WindowId, Result<Page, Error>)
}