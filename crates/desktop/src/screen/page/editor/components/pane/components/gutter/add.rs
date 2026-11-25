use carreau::Rect;
use glyphon::{FontSystem, TextArea};

use crate::screen::page::{Palette, editor::{Layout, components::pane::Gutter}};

impl Gutter {
    pub(crate) fn add<'a>(
        &'a mut self,
        areas: &mut Vec<TextArea<'a>>,
        fs: &mut FontSystem,
        layout: Layout,
        palette: &Palette,
        rects: &mut Vec<Rect>,
        x: f32
    ) {
        self.numbers.0.set_size(Some(self.width), Some(layout.h - 89.0));
        self.numbers.0.shape_until_scroll(fs, false);

        areas.push((self.numbers.1)(&self.numbers.0, layout, palette.ink.text(), x));
        
        rects.push((self.rule)(layout, palette.sb.rect(), x));
    }
}
