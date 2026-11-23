use glyphon::FontSystem;

use crate::screen::page::editor::components::Body;

impl Body {
    pub(crate) fn shape(
        &mut self,
        fs: &mut FontSystem,
        height: f32
    ) {
        self.text.0.set_size(None, Some(height));
        self.text.0.shape_until_scroll(fs, false);
    }
}
