mod area;
mod rect;
mod shape;

use std::fmt::Write;

use carreau::Rect;
use glyphon::{Attrs, Buffer, Family, Metrics, Shaping, cosmic_text::Align};

use crate::screen::page::{Palette, editor::{Area, Layout}};

pub(crate) struct Gutter {
    numbers: (Buffer, Area),
    rule: fn(Layout, [f32; 4], f32) -> Rect,
    width: f32
}

impl Gutter {
    pub(crate) fn new(
        active: usize,
        numbers: Area,
        palette: &Palette,
        rule: fn(Layout, [f32; 4], f32) -> Rect,
        text: &[u8],
        width: f32
    ) -> Self {
        let mut buffer = Buffer::new_empty(Metrics::new(12.5, 26.0));

        let ink = Attrs::new().family(Family::Name("Cascadia Code"));
        let faint = ink.clone().color(palette.gutter.text());
        
        let count = text.iter().filter(|&&byte| byte == b'\n').count() + 1;

        let mut left = String::with_capacity(active - 1);
        for i in 1..=active - 1 {
            writeln!(left, "{}", i).unwrap();
        }
        
        let middle = if active == count {
            format!("{}", active)
        }
        else {
            format!("{}\n", active)
        };
        
        let mut right = String::with_capacity(count - active);
        for i in (active + 1)..=count {
            if i + 1 > count {
                write!(right, "{}", i).unwrap();
            }
            else {
                writeln!(right, "{}", i).unwrap();
            }
        }

        buffer.set_rich_text([
            (left.as_str(), faint.clone()),
            (middle.as_str(), ink.clone()),
            (right.as_str(), faint.clone())
        ], &faint, Shaping::Advanced, Some(Align::Right));

        Self {
            numbers: (buffer, numbers),
            rule,
            width
        }
    }
}
