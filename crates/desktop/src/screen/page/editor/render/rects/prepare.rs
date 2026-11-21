use std::sync::{Arc, RwLock};

use carreau::{Rect, rect::Border};

use crate::screen::{Resources, page::editor::Layout};

pub(crate) fn prepare(
    layout: Layout,
    rects: Arc<RwLock<Vec<Rect>>>,
    resources: Arc<Resources>
) {
    let Layout { h, s, w } = layout;

    // Doc heading
    let dh = Rect {
        backdrop: [233.0 / 255.0, 225.0 / 255.0, 213.0 / 255.0, 1.0],
        border: Border::new(
            [0.0; 4],
            0.0,
            0.0
        ),
        position: [310.0 * s, 65.0 * s],
        size: [(w - 310.0) * s, 2.0 * s]
    };

    let divider = Rect {
        backdrop: [215.0 / 255.0, 205.0 / 255.0, 188.0 / 255.0, 1.0],
        border: Border::new(
            [0.0; 4],
            0.0,
            0.0
        ),
        position: [308.0 * s, 0.0],
        size: [2.0 * s, h * s]
    };

    // Doc line numbers
    let dln = Rect {
        backdrop: [233.0 / 255.0, 225.0 / 255.0, 213.0 / 255.0, 1.0],
        border: Border::new(
            [0.0; 4],
            0.0,
            0.0
        ),
        position: [371.0 * s, 67.0 * s],
        size: [2.0 * s, (h - 67.0) * s]
    };

    // Filesystem heading
    let fh = Rect {
        backdrop: [233.0 / 255.0, 225.0 / 255.0, 213.0 / 255.0, 1.0],
        border: Border::new(
            [0.0; 4],
            0.0,
            0.0
        ),
        position: [0.0, 65.0 * s],
        size: [308.0 * s, 2.0 * s]
    };

    // Filesystem line numbers
    let fln = Rect {
        backdrop: [233.0 / 255.0, 225.0 / 255.0, 213.0 / 255.0, 1.0],
        border: Border::new(
            [0.0; 4],
            0.0,
            0.0
        ),
        position: [38.0 * s, 67.0 * s],
        size: [2.0 * s, (h - 67.0) * s]
    };

    rects.write().unwrap().extend([
        divider,
        fh,
        dh,
        fln,
        dln
    ]);
    
    let rects = rects.read().unwrap();
    resources.carreau.write().unwrap().prepare(&resources.gpu.queue, &rects);
}
