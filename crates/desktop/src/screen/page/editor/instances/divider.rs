use carreau::{Rect, rect::Border};

use crate::screen::page::editor::Divider;

pub(crate) fn divider() -> Divider {
    Divider::new(
        |layout, backdrop| Rect {
            backdrop,
            border: Border::new([0.0; 4], 0.0, 0.0),
            position: [(layout.fw - 2.0) * layout.s, 0.0],
            size: [6.0 * layout.s, layout.h * layout.s]
        },
        |layout, backdrop| Rect {
            backdrop,
            border: Border::new([0.0; 4], 0.0, 0.0),
            position: [layout.fw * layout.s, 0.0],
            size: [2.0 * layout.s, layout.h * layout.s]
        }
    )
}