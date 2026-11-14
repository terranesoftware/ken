use glyphon::{Attrs, Buffer, Color, Metrics, Shaping};
use rand::seq::IndexedRandom;

use crate::page::landing::buffers::FAMILIES;

pub(super) fn name() -> Buffer {
    let mut buffer = Buffer::new_empty(Metrics::new(200.0, f32::MIN_POSITIVE));

    let (family, weight) = *FAMILIES.choose(&mut rand::rng()).unwrap();
    let attrs = Attrs::new().family(family).weight(weight);

    buffer.set_rich_text([
        ("ken", attrs.clone().color(Color::rgb(41, 40, 35))),
        (".", attrs.clone().color(Color::rgb(183, 60, 50)))
    ], &attrs, Shaping::Advanced, None);

    buffer
}