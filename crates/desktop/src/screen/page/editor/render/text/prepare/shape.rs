use glyphon::FontSystem;

use crate::screen::page::editor::Buffers;

pub(super) fn shape(
    buffers: &mut Buffers,
    fs: &mut FontSystem
) {
    buffers.shape(fs);
}