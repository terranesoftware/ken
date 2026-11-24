use glyphon::{Color, TextArea};

use crate::screen::page::editor::{Layout, components::pane::Heading};

impl Heading {
    pub(crate) fn areas(
        &self,
        layout: Layout,
        ink: Color,
        x: f32
    ) -> [TextArea<'_>; 2] {
        [
            (self.name.1)(&self.name.0, layout, ink, x),
            (self.path.1)(&self.path.0, layout, ink, x)
        ]
    }
}
