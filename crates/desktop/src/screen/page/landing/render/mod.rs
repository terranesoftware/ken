pub(super) mod rects;

pub(super) mod text;

use std::sync::Arc;

use tokio::task::spawn_blocking;

use crate::{Error, screen::{Resources, page::{Landing, Palette, landing::Layout}}};

impl Landing {
    pub(crate) async fn prepare(
        self: &Arc<Self>,
        layout: Layout,
        palette: Palette,
        resources: &Arc<Resources>,
    ) -> Result<(), Error> {
        *self.layout.write().unwrap() = layout;

        let ml = self.clone();
        let mres = resources.clone();
        let rects = spawn_blocking(move || rects::prepare(&ml, layout, palette, &mres));

        let ml = self.clone();
        let mres = resources.clone();
        let text = spawn_blocking(move || text::prepare(&ml, layout, palette, mres));

        rects.await?;
        text.await?
    }
}