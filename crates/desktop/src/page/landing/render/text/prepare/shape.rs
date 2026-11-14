use glyphon::FontSystem;

use crate::page::Buffers;

pub(super) fn shape(
    buffers: &mut Buffers,
    fs: &mut FontSystem
) {
    buffers.shape(fs);
}