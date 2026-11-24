use carreau::Rect;

use crate::screen::page::{Palette, editor::{Layout, components::pane::Cursor}};

impl Cursor {
    pub(crate) fn rects(
        &self,
        layout: Layout,
        palette: &Palette,
        x: f32
    ) -> [Rect; 2] {
        [
            (self.band)(layout, palette, x),
            (self.caret)(layout, palette.accent.rect(), x)
        ]
    }
}
