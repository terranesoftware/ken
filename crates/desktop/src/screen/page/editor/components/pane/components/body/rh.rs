use crate::screen::page::editor::{Layout, components::pane::Body};

impl Body {
    // Rect hit
    pub(crate) fn rh(
        &self,
        layout: Layout,
        point: [f32; 2],
        x: f32
    ) -> bool {
        (self.rect)(layout, [0.0; 4], x).hit(point)
    }
}
