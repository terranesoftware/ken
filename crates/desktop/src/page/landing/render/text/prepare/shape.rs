use glyphon::FontSystem;

use crate::page::landing::Buffers;

pub(super) fn shape(
    buffers: &mut Buffers,
    fs: &mut FontSystem
) {
    buffers.shape(fs);
}