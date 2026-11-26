use carreau::Rect;
use glyphon::{Cursor, FontSystem, TextArea};

use crate::screen::page::{Palette, editor::{Layout, components::pane::Body}};

impl Body {
    pub(crate) fn add<'a>(
        &'a mut self,
        areas: &mut Vec<TextArea<'a>>,
        (line, index): (usize, usize),
        fs: &mut FontSystem,
        layout: Layout,
        palette: &Palette,
        rects: &mut Vec<Rect>,
        x: f32
    ) {
        self.text.0.set_size(None, Some(layout.h - 89.0));
        self.text.0.shape_until_scroll(fs, false);

        areas.push((self.text.1)(&self.text.0, layout, palette.ink.text(), x));

        if let Some((cx, top)) = self.text.0.cursor_position(&Cursor::new(line, index)) {
            self.cursor.add(layout, palette, [cx, top], rects, x);
        }

        let active = line + 1;
        let count = self.text.0.lines.len();
        self.gutter.add(active, areas, count, fs, layout, palette, rects, x);
    }
}
