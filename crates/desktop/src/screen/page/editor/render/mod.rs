mod area;
pub(super) use area::area;

use std::sync::Arc;

use ken::buffers::File;
use tokio::task::block_in_place;

use crate::{Error, screen::{Resources, page::{Editor, Palette, editor::Layout}}};

impl Editor {
    pub(crate) async fn prepare(
        self: &Arc<Self>,
        layout: Layout,
        palette: Palette,
        resources: &Arc<Resources>
    ) -> Result<(), Error> {
        *self.layout.write().unwrap() = layout;

        block_in_place(|| {
            let ken = self.ken.read().unwrap();
            let mut fs = resources.text.fs.write().unwrap();

            let mut filesystem = self.filesystem.write().unwrap();
            let mut file = self.file.write().unwrap();

            let mut areas = Vec::new();
            let mut rects = Vec::new();

            // Add the filesystem pane and the divider *and then* return the divider's edge so the file pane knows where to start
            // Otherwise, the file pane starts all the way from the left
            let x = filesystem.as_mut().map_or(0.0, |filesystem| {
                let directories = &ken.directories;
                let cursor = directories.map.get(directories.active.as_ref().unwrap()).unwrap().cursor;
                
                filesystem.add(&mut areas, cursor, &mut fs, layout, &palette, &mut rects, 0.0);
                self.divider.add(layout, &palette, &mut rects);
                
                layout.de()
            });

            if let Some(file) = file.as_mut() {
                let files = &ken.files;
                let cursor = match files.active.as_ref().unwrap() {
                    File::Ephemeral(key) => files.ephemeral.get(*key),
                    File::Persistent(path) => files.persistent.get(path)
                }.unwrap().cursor;

                file.add(&mut areas, cursor, &mut fs, layout, &palette, &mut rects, x);
            }

            resources.rects(&rects);
            resources.areas(areas, &mut fs)
        })
    }
}
