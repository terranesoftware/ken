mod add;

use carreau::Rect;
use glyphon::{Buffer, Metrics};

use crate::screen::page::editor::{Area, Layout};

pub(crate) struct Gutter {
    numbers: (Buffer, Area),
    rule: fn(Layout, [f32; 4], f32) -> Rect,
    width: f32
}

impl Gutter {
    pub(crate) fn new(
        area: Area,
        rule: fn(Layout, [f32; 4], f32) -> Rect,
        width: f32
    ) -> Self {
        Self {
            numbers: (Buffer::new_empty(Metrics::new(12.5, 26.0)), area),
            rule,
            width
        }
    }
}
