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
        let layout = *self.layout.read().unwrap();
        let point = [position.x as f32, position.y as f32];

        // Checked first since it overlaps the filesystem hit rect, gives the "simulation" of a higher z-level
        if !layout.focused && self.divider.hit(layout, point) {
            CursorIcon::ColResize
        }
        // The filesystem pane is absent if focused
        else if self.file.as_ref().is_some_and(|file| file.read().unwrap().hit(layout, point)) || (!layout.focused && self.filesystem.read().unwrap().hit(layout, point)) {
            CursorIcon::Text
        }
        else {
            CursorIcon::Default
        }
    }
}