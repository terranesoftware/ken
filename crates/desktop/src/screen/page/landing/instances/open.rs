use std::sync::RwLock;

use carreau::{Rect, rect::Border};

use crate::screen::{Palette, landing::{Action, render::text::{area, width}}};

pub(crate) fn open(palette: &Palette) -> RwLock<Action> {
    RwLock::new(Action::new(
        ("\u{E247}", |buffer, layout, ink| area(buffer, layout.left + 12.5, layout.top + 250.0, layout.s, ink)),
        ("Open", |buffer, layout, ink| area(buffer, layout.left + 50.0, layout.top + 250.0, layout.s, ink)),
        palette,
        |layout, backdrop| Rect {
            backdrop,
            border: Border::new([0.0; 4], 11.375 * layout.s, 0.0),
            position: [layout.left * layout.s, (layout.top + 212.5) * layout.s],
            size: [layout.cw * layout.s, 75.0 * layout.s]
        },
        ("Ctrl+O", |buffer, layout, ink| area(buffer, layout.right - width(buffer) - 12.5, layout.top + 250.0, layout.s, ink))
    ))
}