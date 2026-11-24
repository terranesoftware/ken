use glyphon::{Color, TextArea};

use crate::screen::page::editor::{Layout, components::Pane};

impl Pane {
    pub(crate) fn areas(
        &self,
        layout: Layout,
        ink: Color,
        x: f32
    ) -> [TextArea<'_>; 4] {
        let [name, path] = self.heading.areas(layout, ink, x);

        [
            self.body.area(layout, ink, x),
            self.gutter.area(layout, ink, x),
            name,
            path
        ]
    }
}
