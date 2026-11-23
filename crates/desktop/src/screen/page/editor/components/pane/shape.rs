use glyphon::FontSystem;

use crate::screen::page::editor::components::Pane;

impl Pane {
    pub(crate) fn shape(
        &mut self,
        fs: &mut FontSystem,
        height: f32
    ) {
        self.body.shape(fs, height);
        self.gutter.shape(fs, height);
        self.heading.shape(fs);
    }
}
