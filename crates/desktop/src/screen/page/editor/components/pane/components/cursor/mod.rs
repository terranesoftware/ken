mod rects;

use carreau::Rect;

use crate::screen::page::{Palette, editor::Layout};

pub(crate) struct Cursor {
    band: fn(Layout, &Palette, f32) -> Rect,
    caret: fn(Layout, [f32; 4], f32) -> Rect
}

impl Cursor {
    pub(crate) fn new(
        band: fn(Layout, &Palette, f32) -> Rect,
        caret: fn(Layout, [f32; 4], f32) -> Rect
    ) -> Self {
        Self {
            band,
            caret
        }
    }
}
