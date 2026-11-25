mod add;
mod hit;

use carreau::Rect;

use crate::screen::page::editor::Layout;

pub(crate) struct Divider {
    hit: fn(Layout, [f32; 4]) -> Rect,
    rule: fn(Layout, [f32; 4]) -> Rect
}

impl Divider {
    pub(crate) fn new(
        hit: fn(Layout, [f32; 4]) -> Rect,
        rule: fn(Layout, [f32; 4]) -> Rect
    ) -> Self {
        Self {
            hit,
            rule
        }
    }
}
