use crate::screen::page::editor::{Layout, components::Pane};

impl Pane {
    pub(crate) fn hit(
        &self,
        layout: Layout,
        point: [f32; 2],
        x: f32
    ) -> bool {
        self.body.rh(layout, point, x)
    }
}
