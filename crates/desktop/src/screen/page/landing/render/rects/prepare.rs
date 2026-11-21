use std::sync::{Arc, RwLock};

use carreau::{Rect, rect::Border};

use crate::screen::{Resources, page::{Palette, landing::Layout}};

pub(crate) fn prepare(
    layout: Layout,
    palette: Palette,
    rects: Arc<RwLock<Vec<Rect>>>,
    resources: Arc<Resources>
) {
    let Layout { cw, left, s, top, .. } = layout;

    // Hover backdrops start transparent; `moved` shows them
    let [r, g, b, _] = palette.hover.rect();
    let hover = [r, g, b, 0.0];
    
    let nb = Rect {
        backdrop: hover,
        border: Border::new(
            [0.0; 4],
            11.375 * s,
            0.0
        ),
        position: [left * s, (top + 137.5) * s],
        size: [cw * s, 75.0 * s]
    };

    let ob = Rect {
        backdrop: hover,
        border: Border::new(
            [0.0; 4],
            11.375 * s,
            0.0
        ), 
        position: [left * s, (top + 212.5) * s],
        size: [cw * s, 75.0 * s]
    };

    let separator = Rect {
        backdrop: palette.border.rect(),
        border: Border::new(
            [0.0; 4],
            0.0,
            0.0
        ),
        position: [left * s, (top + 312.5) * s],
        size: [cw * s, 2.5 * s]
    };

    *rects.write().unwrap() = vec![
        nb,
        ob,
        separator
    ];
    
    let rects = rects.read().unwrap();
    resources.carreau.write().unwrap().prepare(&resources.gpu.queue, &rects);
}