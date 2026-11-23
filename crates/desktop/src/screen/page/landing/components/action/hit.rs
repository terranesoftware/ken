use crate::screen::landing::{Action, Layout};

impl Action {
    pub(crate) fn hit(
        &self,
        layout: Layout,
        point: [f32; 2]
    ) -> bool {
        (self.rect)(layout, [0.0; 4]).hit(point)
    }
}