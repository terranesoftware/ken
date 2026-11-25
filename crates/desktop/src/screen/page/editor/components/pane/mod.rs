mod add;

pub(crate) mod components;
use components::{Body, Gutter, Heading};

mod hit;

use carreau::Rect;

use crate::screen::page::{Palette, editor::Layout};

pub(crate) struct Pane {
    pub(crate) background: fn(Layout, &Palette, f32) -> Rect,
    pub(crate) body: Body,
    pub(crate) gutter: Gutter,
    pub(crate) heading: Heading
}