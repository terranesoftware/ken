use std::sync::RwLock;

use glyphon::{Attrs, Buffer, Family, Metrics, Shaping, Weight};

use crate::screen::landing::{Area, render::text::area};

pub(crate) fn re() -> RwLock<(Buffer, Area)> {
    RwLock::new((
        {
            let mut buffer = Buffer::new_empty(Metrics::new(17.5, 10.0));
            
            let attrs = Attrs::new().family(Family::Name("Gelasio")).weight(Weight::NORMAL);
            
            buffer.set_text("Nothing here yet.", &attrs, Shaping::Advanced, None);
            
            buffer
        },
        |buffer, layout, ink| area(buffer, layout.left, layout.top + 425.0, layout.s, ink)
    ))
}