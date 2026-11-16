use glyphon::{Attrs, Buffer, Color, Family, Metrics, Shaping, Weight};

pub(super) fn oi() -> Buffer {
    let mut buffer = Buffer::new_empty(Metrics::new(40.0, f32::MIN_POSITIVE));

    let attrs = Attrs::new().color(Color::rgb(183, 60, 50)).family(Family::Name("lucide")).weight(Weight::NORMAL);

    buffer.set_text("\u{E247}", &attrs, Shaping::Advanced, None);

    buffer
}