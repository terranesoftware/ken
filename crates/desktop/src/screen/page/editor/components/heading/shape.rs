use glyphon::FontSystem;

use crate::screen::page::editor::components::Heading;

impl Heading {
    pub(crate) fn shape(
        &mut self,
        fs: &mut FontSystem
    ) {
        self.name.0.shape_until_scroll(fs, false);
        self.path.0.shape_until_scroll(fs, false);
    }
}
