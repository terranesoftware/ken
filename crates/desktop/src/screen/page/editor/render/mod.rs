pub(super) mod rects;

pub(super) mod text;

use std::sync::Arc;

use tokio::task::spawn_blocking;

use crate::{Error, screen::{Resources, page::{Editor, editor::Layout}}};

impl Editor {
    pub(crate) async fn prepare(
        &self,
        layout: Layout,
        resources: &Arc<Resources>
    ) -> Result<(), Error> {
        let mrec = self.rects.clone();
        let mres = resources.clone();
        let rects = spawn_blocking(move || rects::prepare(layout, mrec, mres));

        let mb = self.buffers.clone();
        let mres = resources.clone();
        let text = spawn_blocking(move || text::prepare(mb, layout, mres));

        rects.await?;
        text.await?
    }
}