pub(super) mod rects;

pub(super) mod text;

use std::sync::Arc;

use tokio::task::{block_in_place, spawn_blocking};

use crate::{Error, screen::{Resources, page::{Editor, Palette, editor::Layout}}};

impl Editor {
    pub(crate) async fn prepare(
        self: &Arc<Self>,
        layout: Layout,
        palette: Palette,
        resources: &Arc<Resources>
    ) -> Result<(), Error> {
        *self.layout.write().unwrap() = layout;

        // Have to shape before we're able to determine cursor position
        block_in_place(|| {
            let mut fs = resources.text.fs.write().unwrap();
            if let Some(file) = self.file.write().unwrap().as_mut() {
                file.shape(&mut fs, layout.h - 89.0);
            }
            if let Some(filesystem) = self.filesystem.write().unwrap().as_mut() {
                filesystem.shape(&mut fs, layout.h - 89.0);
            }
        });
        
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
