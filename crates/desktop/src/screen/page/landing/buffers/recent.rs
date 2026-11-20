use glyphon::{Attrs, Buffer, Family, Metrics, Shaping, Weight};

pub(super) fn recent() -> Buffer {
    let mut buffer = Buffer::new_empty(Metrics::new(12.5, 8.0));

    let attrs = Attrs::new().family(Family::Name("Cascadia Code")).weight(Weight::SEMIBOLD);

    buffer.set_text("RECENT", &attrs, Shaping::Advanced, None);

    buffer
}