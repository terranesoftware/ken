use glyphon::{Attrs, Buffer, Family, Metrics, Shaping, Weight};

pub(super) fn new() -> Buffer {
    let mut buffer = Buffer::new_empty(Metrics::new(40.0, f32::MIN_POSITIVE));

    let attrs = Attrs::new().family(Family::Name("Inter")).weight(Weight::SEMIBOLD);

    buffer.set_text("New", &attrs, Shaping::Advanced, None);

    buffer
}