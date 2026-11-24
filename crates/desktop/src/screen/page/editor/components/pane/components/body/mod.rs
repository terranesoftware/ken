mod area;
mod hit;
mod shape;

use carreau::Rect;
use glyphon::{Attrs, Buffer, Family, Metrics, Shaping, Weight};

use crate::screen::page::editor::{Area, Layout};

pub(crate) struct Body {
    hit: fn(Layout, [f32; 4], f32) -> Rect,
    text: (Buffer, Area)
}

impl Body {
    pub(crate) fn new(
        hit: fn(Layout, [f32; 4], f32) -> Rect,
        text: (&str, Area)
    ) -> Self {
        // LISTING CODE THAT I ACCIDENTALLY PUT HERE
        /* let mut buffer = Buffer::new_empty(Metrics::new(12.5, 26.0));
        
        let file = Attrs::new().color(palette.muted.text()).family(Family::Name("Cascadia Code"));
        let folder = Attrs::new().family(Family::Name("Cascadia Code")).weight(Weight::SEMIBOLD);
        
        let mut spans = Vec::new();
        
        for line in text.0.split_inclusive('\n') {
            let (entry, newline) = line.split_at(line.trim_end().len());
            
            // A trailing slash marks a folder, which takes bold ink
            spans.push((entry, if entry.ends_with('/') { folder.clone() } else { file.clone() }));
            
            if !newline.is_empty() {
                spans.push((newline, file.clone()));
            }
        }
        
        buffer.set_rich_text(spans, &file, Shaping::Advanced, None); */

        let mut buffer = Buffer::new_empty(Metrics::new(12.5, 26.0));
        
        let attrs = Attrs::new().family(Family::Name("Cascadia Code")).weight(Weight::NORMAL);
        
        buffer.set_text(text.0, &attrs, Shaping::Advanced, None);
        
        Self {
            hit,
            text: (buffer, text.1)
        }
    }
}
