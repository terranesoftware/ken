use winit::window::WindowId;

use crate::{Error, Screen};

pub(super) enum UserEvent {
    Render(WindowId, Result<(), Error>),
    Screen(WindowId, Result<Screen, Error>)
}