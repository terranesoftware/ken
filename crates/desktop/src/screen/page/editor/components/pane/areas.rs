use glyphon::{Color, TextArea};

use crate::screen::page::editor::{Layout, components::Pane};

impl Pane {
    pub(crate) fn areas(
        &self,
        layout: Layout,
        ink: Color
    ) -> [TextArea<'_>; 4] {
        let [name, path] = self.heading.areas(layout, ink);

        [
            self.body.area(layout, ink),
            self.gutter.area(layout, ink),
            name,
            path
        ]
    }
}
