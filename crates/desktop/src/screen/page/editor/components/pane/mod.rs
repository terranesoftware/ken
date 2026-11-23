mod areas;
mod hit;
mod rects;
mod shape;

use carreau::Rect;

use crate::screen::page::{Palette, editor::{Layout, components::{Body, Cursor, Gutter, Heading}}};

pub(crate) struct Pane {
    background: fn(Layout, &Palette) -> Rect,
    body: Body,
    cursor: Cursor,
    gutter: Gutter,
    heading: Heading
}

impl Pane {
    pub(crate) fn new(
        background: fn(Layout, &Palette) -> Rect,
        body: Body,
        cursor: Cursor,
        gutter: Gutter,
        heading: Heading
    ) -> Self {
        Self {
            background,
            body,
            cursor,
            gutter,
            heading
        }
    }
}
