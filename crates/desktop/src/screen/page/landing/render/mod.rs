mod background;
pub(super) use background::background;

mod layout;
use layout::layout;

pub(super) mod rects;

pub(super) mod text;

use std::sync::{Arc, OnceLock};

use tokio::task::spawn_blocking;
use winit::dpi::PhysicalSize;

use crate::{Error, screen::{Resources, page::Landing}};

static LAYOUT: OnceLock<(f32, f32, f32, f32, f32)> = OnceLock::new();

impl Landing {
    pub(crate) async fn prepare(
        &self,
        resources: &Arc<Resources>,
        size: PhysicalSize<u32>
    ) -> Result<(), Error> {
        _ = LAYOUT.set(layout(size));

        let mrec = self.rects.clone();
        let mres = resources.clone();
        let rects = spawn_blocking(|| rects::prepare(mrec, mres));

        let mb = self.buffers.clone();
        let mres = resources.clone();
        let text = spawn_blocking(|| text::prepare(mb, mres));

        rects.await?;
        text.await?
    }
}