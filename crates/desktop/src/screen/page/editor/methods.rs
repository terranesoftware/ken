use std::sync::Arc;

use winit::{dpi::PhysicalPosition, window::{CursorIcon, Window}};

use crate::screen::{Destination, PageMethods, Palette, Resources, page::Editor};

impl PageMethods for Arc<Editor> {
    fn clicked(
        &self,
        position: PhysicalPosition<f64>,
        window: &Window
    ) -> Option<Destination> {
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