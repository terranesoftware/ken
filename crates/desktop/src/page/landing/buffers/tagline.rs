use glyphon::{Attrs, Buffer, Metrics, Shaping};
use rand::seq::IndexedRandom;

use crate::page::landing::buffers::FAMILIES;

pub(super) fn tagline() -> Buffer {
    let mut buffer = Buffer::new_empty(Metrics::new(75.0, f32::MIN_POSITIVE));
    
    let (family, weight) = *FAMILIES.choose(&mut rand::rng()).unwrap();
    let attrs = Attrs::new().family(family).weight(weight);

    buffer.set_text("The text editor is back.", &attrs, Shaping::Advanced, None);

    buffer
}