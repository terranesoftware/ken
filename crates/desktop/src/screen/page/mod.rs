mod background;
use background::background;

mod destination;

pub(crate) use destination::Destination;

pub(crate) mod editor;
pub(super) use editor::Editor;

pub(crate) mod landing;
pub(super) use landing::Landing;

use std::sync::Arc;

use enum_dispatch::enum_dispatch;
use winit::dpi::PhysicalPosition;

use crate::{Error, screen::{Resources, palette::Palette}};

#[enum_dispatch(PageMethods)]
pub(crate) enum Page {
    Editor(Arc<Editor>),
    Landing(Arc<Landing>)
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
        palette: &Palette,
        resources: &Arc<Resources>
    ) -> Result<(), Error>;
}