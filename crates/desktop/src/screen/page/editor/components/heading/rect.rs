use carreau::Rect;

use crate::screen::page::{Palette, editor::{Layout, components::Heading}};

impl Heading {
    pub(crate) fn rect(
        &self,
        layout: Layout,
        palette: &Palette
    ) -> Rect {
        (self.rule)(layout, palette.sb.rect())
    }
}
