use carreau::Rect;

use crate::screen::page::{Palette, editor::{Layout, components::Pane}};

impl Pane {
    pub(crate) fn rects(
        &self,
        layout: Layout,
        palette: &Palette,
        x: f32
    ) -> [Rect; 5] {
        let [band, caret] = self.cursor.rects(layout, palette, x);

        [
            (self.background)(layout, palette, x),
            band,
            caret,
            self.gutter.rect(layout, palette, x),
            self.heading.rect(layout, palette, x)
        ]
    }
}
