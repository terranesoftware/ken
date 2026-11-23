use std::sync::RwLock;

use glyphon::{Attrs, Buffer, Metrics, Shaping};
use rand::seq::IndexedRandom;

use crate::screen::landing::{Area, FAMILIES, render::text::{area, center}};

pub(crate) fn tagline() -> RwLock<(Buffer, Area)> {
    RwLock::new((
        {
            let mut buffer = Buffer::new_empty(Metrics::new(37.5, f32::MIN_POSITIVE));
            
            let (family, weight) = *FAMILIES.choose(&mut rand::rng()).unwrap();
            let attrs = Attrs::new().family(family).weight(weight);
            
            buffer.set_text("The text editor is back.", &attrs, Shaping::Advanced, None);
            
            buffer
        },
        |buffer, layout, ink| area(buffer, center(buffer, layout.w), layout.top + 87.5, layout.s, ink)
    ))
}