mod area;
mod hit;
mod shape;

use carreau::Rect;
use glyphon::Buffer;

use crate::screen::page::editor::{Area, Layout};

pub(crate) struct Body {
    hit: fn(Layout, [f32; 4]) -> Rect,
    text: (Buffer, Area)
}

impl Body {
    pub(crate) fn new(
        hit: fn(Layout, [f32; 4]) -> Rect,
        text: (Buffer, Area)
    ) -> Self {
        Self {
            hit,
            text
        }
    }
}
