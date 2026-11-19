use glyphon::FontSystem;

use crate::page::editor::Buffers;

pub(super) fn shape(
    buffers: &mut Buffers,
    fs: &mut FontSystem
) {
    buffers.shape(fs);
}