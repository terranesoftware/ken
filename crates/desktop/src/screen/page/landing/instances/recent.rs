use std::sync::RwLock;

use glyphon::{Attrs, Buffer, Family, Metrics, Shaping, Weight};

use crate::screen::landing::{Area, render::text::area};

pub(crate) fn recent() -> RwLock<(Buffer, Area)> {
    RwLock::new((
        {
            let mut buffer = Buffer::new_empty(Metrics::new(12.5, 8.0));
            
            let attrs = Attrs::new().family(Family::Name("Cascadia Code")).weight(Weight::SEMIBOLD);
            
            buffer.set_text("RECENT", &attrs, Shaping::Advanced, None);
            
            buffer
        },
        |buffer, layout, ink| area(buffer, layout.left, layout.top + 375.0, layout.s, ink)
    ))
}