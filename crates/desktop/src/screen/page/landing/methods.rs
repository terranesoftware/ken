use std::sync::Arc;

use rfd::FileDialog;
use winit::{dpi::PhysicalPosition, window::{CursorIcon, Window}};

use crate::screen::{Destination, PageMethods, Palette, Resources, landing::render::rects, page::Landing};

impl PageMethods for Arc<Landing> {
    fn clicked(
        &self,
        position: PhysicalPosition<f64>,
        window: &Window
    ) -> Option<Destination> {
        let layout = *self.layout.read().unwrap();
        let point = [position.x as f32, position.y as f32];

        if self.new.read().unwrap().hit(layout, point) {
            return Some(Destination::Editor(None));
        }
        else if self.open.read().unwrap().hit(layout, point) {
            return FileDialog::new().set_parent(window).pick_folder().map(|path| Destination::Editor(Some(path)));
        }
        else {
            None
        }
    }
    
    fn moved(
        &self,
        palette: &Palette,
        position: PhysicalPosition<f64>,
        resources: &Resources
    ) -> CursorIcon {
        let layout = *self.layout.read().unwrap();
        let point = [position.x as f32, position.y as f32];

        self.new.write().unwrap().hover(layout, point);
        self.open.write().unwrap().hover(layout, point);

        rects::prepare(self, layout, *palette, resources);

        CursorIcon::Default
    }
}