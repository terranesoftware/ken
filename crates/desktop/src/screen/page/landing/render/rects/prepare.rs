use std::sync::{Arc, RwLock};

use carreau::{Rect, rect::Border};

use crate::screen::{Resources, page::landing::render::LAYOUT};

pub(crate) fn prepare(
    rects: Arc<RwLock<Vec<Rect>>>,
    resources: Arc<Resources>
) {
    let (cw, left, _, top, _) = *LAYOUT.get().unwrap();

    let nb = Rect {
        backdrop: [232.0 / 255.0, 216.0 / 255.0, 193.0 / 255.0, 0.0],
        border: Border::new(
            [0.0; 4],
            cw / 64.0,
            0.0
        ),
        position: [left, top + 275.0],
        size: [cw, 150.0]
    };

    let ob = Rect {
        backdrop: [232.0 / 255.0, 216.0 / 255.0, 193.0 / 255.0, 0.0],
        border: Border::new(
            [0.0; 4],
            cw / 64.0,
            0.0
        ), 
        position: [left, top + 425.0],
        size: [cw, 150.0]
    };

    let separator = Rect {
        backdrop: [215.0 / 255.0, 205.0 / 255.0, 188.0 / 255.0, 1.0],
        border: Border::new(
            [0.0; 4],
            0.0,
            0.0
        ),
        position: [left, top + 625.0],
        size: [cw, 5.0]
    };

    rects.write().unwrap().extend([
        nb,
        ob,
        separator
    ]);
    
    let rects = rects.read().unwrap();
    resources.carreau.write().unwrap().prepare(&resources.gpu.queue, &rects);
}