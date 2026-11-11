use glyphon::{Attrs, Buffer, Family, Metrics, Shaping, Weight};

pub(super) fn tagline() -> Buffer {
    let mut buffer = Buffer::new_empty(Metrics::new(13.0, 20.0));

    let attrs = Attrs::new().family(Family::Name("Georgia")).weight(Weight::NORMAL);

    buffer.set_text("The text editor is back.", &attrs, Shaping::Advanced, None);

    buffer
}