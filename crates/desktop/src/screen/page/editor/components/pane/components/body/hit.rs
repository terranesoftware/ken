use crate::screen::page::editor::{Layout, components::pane::Body};

impl Body {
    pub(crate) fn hit(
        &self,
        layout: Layout,
        point: [f32; 2],
        x: f32
    ) -> bool {
        (self.hit)(layout, [0.0; 4], x).hit(point)
    }
}
