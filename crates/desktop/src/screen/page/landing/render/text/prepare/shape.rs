use glyphon::FontSystem;

use crate::screen::page::landing::Buffers;

pub(super) fn shape(
    buffers: &mut Buffers,
    fs: &mut FontSystem
) {
    buffers.shape(fs);
}