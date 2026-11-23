use carreau::Rect;

use crate::screen::page::{Palette, editor::{Layout, components::Cursor}};

impl Cursor {
    pub(crate) fn rects(
        &self,
        layout: Layout,
        palette: &Palette
    ) -> [Rect; 2] {
        [
            (self.band)(layout, palette),
            (self.caret)(layout, palette.accent.rect())
        ]
    }
}
