use std::sync::RwLock;

use carreau::{Rect, rect::Border};

use crate::screen::{Palette, landing::{Action, render::text::{area, width}}};

pub(crate) fn new(palette: &Palette) -> RwLock<Action> {
    RwLock::new(Action::new(
        ("\u{E0C9}", |buffer, layout, ink| area(buffer, layout.left + 12.5, layout.top + 175.0, layout.s, ink)),
        ("New", |buffer, layout, ink| area(buffer, layout.left + 50.0, layout.top + 175.0, layout.s, ink)),
        palette,
        |layout, backdrop| Rect {
            backdrop,
            border: Border::new([0.0; 4], 11.375 * layout.s, 0.0),
            position: [layout.left * layout.s, (layout.top + 137.5) * layout.s],
            size: [layout.cw * layout.s, 75.0 * layout.s]
        },
        ("Ctrl+N", |buffer, layout, ink| area(buffer, layout.right - width(buffer) - 12.5, layout.top + 175.0, layout.s, ink))
    ))
}