mod areas;
mod rect;
mod shape;

use carreau::Rect;
use glyphon::{Attrs, Buffer, Family, Metrics, Shaping, Weight};

use crate::screen::page::{Palette, editor::{Area, Layout}};

pub(crate) struct Heading {
    name: (Buffer, Area),
    path: (Buffer, Area),
    rule: fn(Layout, [f32; 4], f32) -> Rect
}

impl Heading {
    pub(crate) fn new(
        name: (&str, Area),
        palette: &Palette,
        path: (&str, Area),
        rule: fn(Layout, [f32; 4], f32) -> Rect
    ) -> Self {
        let buffer = |size: f32, text: &str, attrs: Attrs| {
            let mut buffer = Buffer::new_empty(Metrics::new(size, f32::MIN_POSITIVE));

            buffer.set_text(text, &attrs, Shaping::Advanced, None);

            buffer
        };

        let font = Attrs::new().family(Family::Name("Cascadia Code"));

        Self {
            name: (buffer(20.0, name.0, font.clone().weight(Weight::SEMIBOLD)), name.1),
            path: (buffer(10.0, path.0, font.color(palette.faint.text())), path.1),
            rule
        }
    }
}
