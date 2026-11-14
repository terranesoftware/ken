use glyphon::{Attrs, Buffer, Family, Metrics, Shaping, Weight};

pub(crate) fn nc() -> Buffer {
    let mut buffer = Buffer::new_empty(Metrics::new(40.0, f32::MIN_POSITIVE));

    let attrs = Attrs::new().family(Family::Name("Cascadia Code")).weight(Weight::NORMAL);

    buffer.set_text("Ctrl+N", &attrs, Shaping::Advanced, None);

    buffer
}