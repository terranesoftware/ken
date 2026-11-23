use carreau::Rect;

use crate::screen::page::{Palette, editor::{Layout, components::Divider}};

impl Divider {
    pub(crate) fn rect(
        &self,
        layout: Layout,
        palette: &Palette
    ) -> Rect {
        (self.rule)(layout, palette.border.rect())
    }
}
