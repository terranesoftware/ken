use crate::screen::page::editor::{Layout, components::Pane};

impl Pane {
    pub(crate) fn hit(
        &self,
        layout: Layout,
        point: [f32; 2]
    ) -> bool {
        self.body.hit(layout, point)
    }
}
