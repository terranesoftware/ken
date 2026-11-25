use carreau::Rect;
use glyphon::{FontSystem, TextArea};

use crate::screen::page::{Palette, editor::{Layout, components::pane::Heading}};

impl Heading {
    pub(crate) fn add<'a>(
        &'a mut self,
        areas: &mut Vec<TextArea<'a>>,
        fs: &mut FontSystem,
        layout: Layout,
        palette: &Palette,
        rects: &mut Vec<Rect>,
        x: f32
    ) {
        self.name.0.shape_until_scroll(fs, false);
        self.path.0.shape_until_scroll(fs, false);

        let ink = palette.ink.text();
        areas.push((self.name.1)(&self.name.0, layout, ink, x));
        areas.push((self.path.1)(&self.path.0, layout, ink, x));

        rects.push((self.rule)(layout, palette.sb.rect(), x));
    }
}
