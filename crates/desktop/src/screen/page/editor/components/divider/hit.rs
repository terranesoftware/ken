use crate::screen::page::editor::{Layout, components::Divider};

impl Divider {
    pub(crate) fn hit(
        &self,
        layout: Layout,
        point: [f32; 2]
    ) -> bool {
        (self.hit)(layout, [0.0; 4]).hit(point)
    }
}
