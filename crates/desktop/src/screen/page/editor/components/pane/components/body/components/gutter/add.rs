use std::fmt::Write;

use carreau::Rect;
use glyphon::{Attrs, Family, FontSystem, Shaping, TextArea, cosmic_text::Align};

use crate::screen::page::{Palette, editor::{Layout, components::pane::components::body::components::Gutter}};

impl Gutter {
    pub(crate) fn add<'a>(
        &'a mut self,
        active: usize,
        areas: &mut Vec<TextArea<'a>>,
        count: usize,
        fs: &mut FontSystem,
        layout: Layout,
        palette: &Palette,
        rects: &mut Vec<Rect>,
        x: f32
    ) {
        let ink = Attrs::new().family(Family::Name("Cascadia Code"));
        let faint = ink.clone().color(palette.gutter.text());

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
        
        self.numbers.0.set_rich_text([
            (left.as_str(), faint.clone()),
            (middle.as_str(), ink.clone()),
            (right.as_str(), faint.clone())
        ], &faint, Shaping::Advanced, Some(Align::Right));
        
        self.numbers.0.set_size(Some(self.width), Some(layout.h - 89.0));
        self.numbers.0.shape_until_scroll(fs, false);

        areas.push((self.numbers.1)(&self.numbers.0, layout, palette.ink.text(), x));
        
        rects.push((self.rule)(layout, palette.sb.rect(), x));
    }
}
