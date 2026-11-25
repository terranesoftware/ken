use carreau::Rect;

use crate::screen::page::{Palette, editor::{Layout, components::pane::components::body::components::Cursor}};

impl Cursor {
    pub(crate) fn add(
        &self,
        layout: Layout,
        palette: &Palette,
        position: [f32; 2],
        rects: &mut Vec<Rect>,
        x: f32
    ) {
        rects.push((self.band)(layout, palette, position[1], x));
        rects.push((self.caret)(layout, palette.accent.rect(), position, x));
    }
}
