use std::sync::{Arc, RwLock};

use carreau::{Carreau, Rect};
use wgpu::Queue;

use crate::page::landing::render::LAYOUT;

pub(crate) fn prepare(
    carreau: Arc<RwLock<Carreau>>,
    queue: Arc<Queue>
) {
    let (cw, left, _, top, _) = *LAYOUT.get().unwrap();
    
    carreau.write().unwrap().prepare(&queue, &[
        Rect {
            backdrop: [215.0 / 255.0, 205.0 / 255.0, 188.0 / 255.0, 1.0],
            position: [left, top + 625.0],
            size: [cw, 5.0]
        }
    ]);
}