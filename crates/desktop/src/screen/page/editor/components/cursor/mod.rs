mod rects;

use carreau::Rect;

use crate::screen::page::{Palette, editor::Layout};

pub(crate) struct Cursor {
    band: fn(Layout, &Palette) -> Rect,
    caret: fn(Layout, [f32; 4]) -> Rect
}

impl Cursor {
    pub(crate) fn new(
        band: fn(Layout, &Palette) -> Rect,
        caret: fn(Layout, [f32; 4]) -> Rect
    ) -> Self {
        Self {
            band,
            caret
        }
    }
}
