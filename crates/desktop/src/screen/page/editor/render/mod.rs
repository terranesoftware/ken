pub(super) mod rects;

pub(super) mod text;

use std::sync::Arc;

use tokio::task::spawn_blocking;

use crate::{Error, screen::{Resources, page::{Editor, Palette, editor::Layout}}};

impl Editor {
    pub(crate) async fn prepare(
        self: &Arc<Self>,
        layout: Layout,
        palette: Palette,
        resources: &Arc<Resources>
    ) -> Result<(), Error> {
        *self.layout.write().unwrap() = layout;

        let me = self.clone();
        let mres = resources.clone();
        let rects = spawn_blocking(move || rects::prepare(&me, layout, palette, &mres));

        let me = self.clone();
        let mres = resources.clone();
        let text = spawn_blocking(move || text::prepare(&me, layout, palette, &mres));

        rects.await?;
        text.await?
    }
}
