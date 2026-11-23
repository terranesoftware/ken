use glyphon::FontSystem;

use crate::screen::page::editor::components::pane::Gutter;

impl Gutter {
    pub(crate) fn shape(
        &mut self,
        fs: &mut FontSystem,
        height: f32
    ) {
        self.numbers.0.set_size(Some(self.width), Some(height));
        self.numbers.0.shape_until_scroll(fs, false);
    }
}
