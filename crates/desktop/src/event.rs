use winit::window::WindowId;

use crate::{Error, Page, app::resources::{Gpu, Text}};

pub(super) enum UserEvent {
    Render(WindowId, Result<(), Error>),
    Resources(Result<(Result<Gpu, Error>, Text), Error>),
    Window(WindowId, Result<Page, Error>)
}