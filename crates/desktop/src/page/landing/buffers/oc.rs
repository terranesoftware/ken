use glyphon::{Attrs, Buffer, Family, Metrics, Shaping, Weight};

pub(super) fn oc() -> Buffer {
    let mut buffer = Buffer::new_empty(Metrics::new(12.0, 18.0));

    let attrs = Attrs::new().family(Family::Name("Cascadia Code")).weight(Weight::NORMAL);

    buffer.set_text("Ctrl+O", &attrs, Shaping::Advanced, None);

    buffer
}