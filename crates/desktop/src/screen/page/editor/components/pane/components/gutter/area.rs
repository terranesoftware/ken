use glyphon::{Color, TextArea};

use crate::screen::page::editor::{Layout, components::pane::Gutter};

impl Gutter {
    pub(crate) fn area(
        &self,
        layout: Layout,
        ink: Color
    ) -> TextArea<'_> {
        (self.numbers.1)(&self.numbers.0, layout, ink)
    }
}
