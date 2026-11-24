use carreau::Rect;

use crate::screen::page::{Palette, editor::{Layout, components::pane::Gutter}};

impl Gutter {
    pub(crate) fn rect(
        &self,
        layout: Layout,
        palette: &Palette,
        x: f32
    ) -> Rect {
        (self.rule)(layout, palette.sb.rect(), x)
    }
}
