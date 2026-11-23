use glyphon::{Color, TextArea};

use crate::screen::page::editor::{Layout, components::Body};

impl Body {
    pub(crate) fn area(
        &self,
        layout: Layout,
        ink: Color
    ) -> TextArea<'_> {
        (self.text.1)(&self.text.0, layout, ink)
    }
}
