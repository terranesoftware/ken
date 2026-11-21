use glyphon::{Attrs, Buffer, Family, Metrics, Shaping, Weight};

use crate::screen::page::Palette;

pub(super) fn ni(palette: &Palette) -> Buffer {
    let mut buffer = Buffer::new_empty(Metrics::new(20.0, f32::MIN_POSITIVE));

    let attrs = Attrs::new().color(palette.accent.text()).family(Family::Name("lucide")).weight(Weight::NORMAL);

    buffer.set_text("\u{E0C9}", &attrs, Shaping::Advanced, None);

    buffer
}