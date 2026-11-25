use carreau::Rect;

use crate::screen::page::{Palette, editor::{Layout, components::Divider}};

impl Divider {
    pub(crate) fn add(
        &self,
        layout: Layout,
        palette: &Palette,
        rects: &mut Vec<Rect>
    ) {
        rects.push((self.rule)(layout, palette.border.rect()));
    }
}
