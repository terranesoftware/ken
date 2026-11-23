use glyphon::FontSystem;

use super::Action;

impl Action {
    pub(crate) fn shape(
        &mut self,
        fs: &mut FontSystem
    ) {
        self.icon.0.shape_until_scroll(fs, false);
        self.label.0.shape_until_scroll(fs, false);
        self.shortcut.0.shape_until_scroll(fs, false);
    }
}