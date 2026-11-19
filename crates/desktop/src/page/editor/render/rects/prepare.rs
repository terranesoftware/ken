use std::sync::{Arc, RwLock};

use carreau::{Carreau, Rect, rect::Border};
use wgpu::Queue;

use crate::page::editor::render::LAYOUT;

pub(crate) fn prepare(
    carreau: Arc<RwLock<Carreau>>,
    queue: Arc<Queue>,
    rects: Arc<RwLock<Vec<Rect>>>
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
    carreau.write().unwrap().prepare(&queue, &rects);
}