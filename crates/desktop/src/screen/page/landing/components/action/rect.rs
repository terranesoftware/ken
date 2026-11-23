use carreau::Rect;

use crate::screen::{Palette, landing::{Layout, components::Action}};

impl Action {
    pub(crate) fn rect(
        &self,
        layout: Layout,
        palette: &Palette
    ) -> Rect {
        // The hover backdrop only shows while hovered
        let [r, g, b, _] = palette.hover.rect();
        
        (self.rect)(layout, [r, g, b, f32::from(self.hovered)])
    }
}