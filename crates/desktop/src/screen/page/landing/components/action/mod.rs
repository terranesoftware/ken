mod areas;
mod hit;
mod hover;
mod rect;
mod shape;

use carreau::Rect;
use glyphon::{Attrs, Buffer, Family, Metrics, Shaping, Weight};

use crate::screen::page::{Palette, landing::{Area, Layout}};

pub(crate) struct Action {
    hovered: bool,
    icon: (Buffer, Area),
    label: (Buffer, Area),
    rect: fn(Layout, [f32; 4]) -> Rect,
    shortcut: (Buffer, Area)
}

impl Action {
    pub(crate) fn new(
        icon: (&str, Area),
        label: (&str, Area),
        palette: &Palette,
        rect: fn(Layout, [f32; 4]) -> Rect,
        shortcut: (&str, Area)
    ) -> Self {
        let buffer = |text: &str, attrs: Attrs| {
            let mut buffer = Buffer::new_empty(Metrics::new(20.0, f32::MIN_POSITIVE));

            buffer.set_text(text, &attrs, Shaping::Advanced, None);

            buffer
        };

        Self {
            hovered: false,
            icon: (buffer(icon.0, Attrs::new().color(palette.accent.text()).family(Family::Name("lucide"))), icon.1),
            label: (buffer(label.0, Attrs::new().family(Family::Name("Inter 28pt")).weight(Weight::SEMIBOLD)), label.1),
            rect,
            shortcut: (buffer(shortcut.0, Attrs::new().family(Family::Name("Cascadia Code"))), shortcut.1)
        }
    }
}