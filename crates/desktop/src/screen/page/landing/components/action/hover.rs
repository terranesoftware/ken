use crate::screen::landing::{Layout, components::Action};

impl Action {
    pub(crate) fn hover(
        &mut self,
        layout: Layout,
        point: [f32; 2]
    ) {
        self.hovered = self.hit(layout, point);
    }
}