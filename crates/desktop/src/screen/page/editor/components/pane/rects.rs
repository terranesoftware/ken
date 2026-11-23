use carreau::Rect;

use crate::screen::page::{Palette, editor::{Layout, components::Pane}};

impl Pane {
    pub(crate) fn rects(
        &self,
        layout: Layout,
        palette: &Palette
    ) -> [Rect; 5] {
        let [band, caret] = self.cursor.rects(layout, palette);

        [
            (self.background)(layout, palette),
            band,
            caret,
            self.gutter.rect(layout, palette),
            self.heading.rect(layout, palette)
        ]
    }
}
