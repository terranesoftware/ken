use glyphon::{Attrs, Buffer, Family, Metrics, Shaping, Weight};

pub(super) fn re() -> Buffer {
    let mut buffer = Buffer::new_empty(Metrics::new(14.0, 20.0));

    let attrs = Attrs::new().family(Family::Name("Georgia")).weight(Weight::NORMAL);

    buffer.set_text("Nothing here yet.", &attrs, Shaping::Advanced, None);

    buffer
}