use glyphon::{Attrs, Buffer, Metrics, Shaping};
use rand::seq::IndexedRandom;

use crate::screen::page::{Palette, landing::buffers::FAMILIES};

pub(super) fn name(palette: &Palette) -> Buffer {
    let mut buffer = Buffer::new_empty(Metrics::new(100.0, f32::MIN_POSITIVE));

    let (family, weight) = *FAMILIES.choose(&mut rand::rng()).unwrap();
    let attrs = Attrs::new().family(family).weight(weight);

    buffer.set_rich_text([
        ("ken", attrs.clone().color(palette.ink.text())),
        (".", attrs.clone().color(palette.accent.text()))
    ], &attrs, Shaping::Advanced, None);

    buffer
}