use carreau::{Rect, rect::Border};

use crate::screen::{Palette, landing::Layout};

pub(crate) fn separator(
    layout: Layout,
    palette: &Palette
) -> Rect {
    Rect {
        backdrop: palette.border.rect(),
        border: Border::new([0.0; 4], 0.0, 0.0),
        position: [layout.left * layout.s, (layout.top + 312.5) * layout.s],
        size: [layout.cw * layout.s, 2.5 * layout.s]
    }
}