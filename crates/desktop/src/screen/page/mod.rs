mod destination;
use std::sync::Arc;

pub(crate) use destination::Destination;

mod editor;
pub(super) use editor::Editor;

mod landing;
pub(super) use landing::Landing;

use enum_dispatch::enum_dispatch;
use winit::dpi::PhysicalPosition;

use crate::{Error, screen::Resources};

#[enum_dispatch(PageMethods)]
pub(crate) enum Page {
    Editor(Editor),
    Landing(Landing)
}

#[enum_dispatch]
pub(crate) trait PageMethods {
    fn clicked(
        &self,
        position: PhysicalPosition<f64>
    ) -> Option<Destination>;

    fn moved(
        &self,
        position: PhysicalPosition<f64>,
        resources: &Resources
    );

    fn render(
        &self,
        resources: &Arc<Resources>
    ) -> Result<(), Error>;
}