use carreau::Rect;
use glyphon::{FontSystem, TextArea};

use crate::screen::page::{Palette, editor::{Layout, components::Pane}};

impl Pane {
    pub(crate) fn add<'a>(
        &'a mut self,
        areas: &mut Vec<TextArea<'a>>,
        cursor: (usize, usize),
        fs: &mut FontSystem,
        layout: Layout,
        palette: &Palette,
        rects: &mut Vec<Rect>,
        x: f32
    ) {
        rects.push((self.background)(layout, palette, x));

        self.body.add(areas, cursor, fs, layout, palette, rects, x);
        self.gutter.add(areas, fs, layout, palette, rects, x);
        self.heading.add(areas, fs, layout, palette, rects, x);
    }
}
