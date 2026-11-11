use glyphon::{Attrs, Buffer, Color, Family, Metrics, Shaping, Weight};
use rand::seq::IndexedRandom;

// Random list of fonts to load, will grow based on what I feel
const FAMILIES: [(Family, Weight); 4] = [
    (Family::Name("Bricolage Grotesque"), Weight::BOLD),
    (Family::Name("Fraunces"), Weight::BOLD),
    (Family::Name("Instrument Serif"), Weight::NORMAL),
    (Family::Name("Righteous"), Weight::NORMAL)
];

pub(super) fn name() -> Buffer {
    let mut buffer = Buffer::new_empty(Metrics::new(40.0, 48.0));

    let (family, weight) = *FAMILIES.choose(&mut rand::rng()).unwrap();
    let attrs = Attrs::new().family(family).weight(weight);

    buffer.set_rich_text([
        ("ken", attrs.clone().color(Color::rgb(41, 40, 35))),
        (".", attrs.clone().color(Color::rgb(183, 60, 50)))
    ], &attrs, Shaping::Advanced, None);

    buffer
}