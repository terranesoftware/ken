use glyphon::{Color, TextArea};

use crate::screen::landing::{Layout, components::Action};

impl Action {
    pub(crate) fn areas(
        &self,
        layout: Layout,
        ink: Color
    ) -> [TextArea<'_>; 3] {
        [
            (self.icon.1)(&self.icon.0, layout, ink),
            (self.label.1)(&self.label.0, layout, ink),
            (self.shortcut.1)(&self.shortcut.0, layout, ink)
        ]
    }
}