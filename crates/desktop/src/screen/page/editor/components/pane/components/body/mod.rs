mod add;

pub(crate) mod components;
use components::{Cursor, Gutter};

mod rh;

mod th;

use carreau::Rect;
use glyphon::{Attrs, Buffer, Family, Metrics, Shaping, Weight};

use crate::screen::page::editor::{Area, Layout};

pub(crate) struct Body {
    cursor: Cursor,
    gutter: Gutter,
    rect: fn(Layout, [f32; 4], f32) -> Rect,
    pub(crate) text: (Buffer, Area)
}

impl Body {
    pub(crate) fn new(
        cursor: Cursor,
        gutter: Gutter,
        rect: fn(Layout, [f32; 4], f32) -> Rect,
        text: (&str, Area)
    ) -> Self {
        let mut buffer = Buffer::new_empty(Metrics::new(12.5, 26.0));
        
        let attrs = Attrs::new().family(Family::Name("Cascadia Code")).weight(Weight::NORMAL);
        
        buffer.set_text(text.0, &attrs, Shaping::Advanced, None);
        
        Self {
            cursor,
            gutter,
            rect,
            text: (buffer, text.1)
        }
    }
}
