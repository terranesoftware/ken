use std::sync::Arc;

use ken::buffers::File;
use winit::{dpi::PhysicalPosition, window::{CursorIcon, Window}};

use crate::screen::{Destination, PageMethods, Palette, Resources, page::Editor};

impl PageMethods for Arc<Editor> {
    fn clicked(
        &self,
        position: PhysicalPosition<f64>,
        _window: &Window
    ) -> Option<Destination> {
        let layout = *self.layout.read().unwrap();
        let point = [position.x as f32, position.y as f32];

        let file = self.file.read().unwrap();
        let filesystem = self.filesystem.read().unwrap();
        if let Some(cursor) = file.as_ref().and_then(|file| file.body.th(layout, point, filesystem.as_ref().map_or(0.0, |_| layout.de()))).or_else(|| {
            filesystem.as_ref().and_then(|filesystem| filesystem.body.th(layout, point, 0.0))
        }) {
            let ken = &mut *self.ken.write().unwrap();

            if let Some(active) = ken.files.active.as_ref() {
                let buffer = match active {
                    File::Ephemeral(key) => ken.files.ephemeral.get_mut(*key).unwrap(),
                    File::Persistent(key) => ken.files.persistent.get_mut(key).unwrap()
                };

                buffer.cursor = (cursor.line, cursor.index);
            }
            if let Some(active) = ken.directories.active.as_ref() {
                ken.directories.map.get_mut(active).unwrap().cursor = (cursor.line, cursor.index);
            }
        }

        None
    }
    
    fn moved(
        &self,
        _palette: &Palette,
        position: PhysicalPosition<f64>,
        _resources: &Resources
    ) -> CursorIcon {
        let filesystem = self.filesystem.read().unwrap();
        let layout = *self.layout.read().unwrap();
        let point = [position.x as f32, position.y as f32];

        // Checked first since it overlaps the filesystem hit rect, gives the "simulation" of a higher z-level
        if filesystem.is_some() && self.divider.hit(layout, point) {
            CursorIcon::ColResize
        }
        else if 
            self.file.read().unwrap().as_ref().is_some_and(|file| file.hit(layout, point, filesystem.as_ref().map_or(0.0, |_| layout.de())))
            ||
            filesystem.as_ref().is_some_and(|filesystem| filesystem.hit(layout, point, 0.0))
        {
            CursorIcon::Text
        }
        else {
            CursorIcon::Default
        }
    }
}