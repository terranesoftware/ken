use glyphon::{Color, Cursor};

use crate::screen::page::editor::{Layout, components::pane::Body};

impl Body {
    // Text hit
    pub(crate) fn th(
        &self,
        layout: Layout,
        point: [f32; 2],
        x: f32
    ) -> Option<Cursor> {
        let area = (self.text.1)(&self.text.0, layout, Color::rgb(0, 0, 0), x);
        
        // Cursor::hit uses coordinates relative to the text buffer, *before* scaling
        self.text.0.hit((point[0] - area.left) / area.scale, (point[1] - area.top) / area.scale)
    }
}
