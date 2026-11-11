use glyphon::{Attrs, Buffer, Family, Metrics, Shaping, Weight};

pub(super) fn new() -> Buffer {
    let mut buffer = Buffer::new_empty(Metrics::new(15.0, 22.0));

    let attrs = Attrs::new().family(Family::Name("Segoe UI")).weight(Weight::SEMIBOLD);

    buffer.set_text("New", &attrs, Shaping::Advanced, None);

    buffer
}