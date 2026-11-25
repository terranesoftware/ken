mod add;

use carreau::Rect;

use crate::screen::page::{Palette, editor::Layout};

pub(crate) struct Cursor {
    pub(crate) band: fn(Layout, &Palette, f32, f32) -> Rect,
    pub(crate) caret: fn(Layout, [f32; 4], [f32; 2], f32) -> Rect
}