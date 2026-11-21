use std::sync::{Arc, RwLock};

use carreau::{Rect, rect::Border};

use crate::screen::{Resources, page::landing::Layout};

pub(crate) fn prepare(
    layout: Layout,
    rects: Arc<RwLock<Vec<Rect>>>,
    resources: Arc<Resources>
) {
    let Layout { cw, left, s, top, .. } = layout;
    
    let nb = Rect {
        backdrop: [240.0 / 255.0, 230.0 / 255.0, 216.0 / 255.0, 0.0],
        border: Border::new(
            [0.0; 4],
            11.375 * s,
            0.0
        ),
        position: [left * s, (top + 137.5) * s],
        size: [cw * s, 75.0 * s]
    };

    let ob = Rect {
        backdrop: [240.0 / 255.0, 230.0 / 255.0, 216.0 / 255.0, 0.0],
        border: Border::new(
            [0.0; 4],
            11.375 * s,
            0.0
        ), 
        position: [left * s, (top + 212.5) * s],
        size: [cw * s, 75.0 * s]
    };

    let separator = Rect {
        backdrop: [215.0 / 255.0, 205.0 / 255.0, 188.0 / 255.0, 1.0],
        border: Border::new(
            [0.0; 4],
            0.0,
            0.0
        ),
        position: [left * s, (top + 312.5) * s],
        size: [cw * s, 2.5 * s]
    };

    rects.write().unwrap().extend([
        nb,
        ob,
        separator
    ]);
    
    let rects = rects.read().unwrap();
    resources.carreau.write().unwrap().prepare(&resources.gpu.queue, &rects);
}