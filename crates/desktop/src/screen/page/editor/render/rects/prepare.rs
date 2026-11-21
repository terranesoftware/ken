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
        position: [310.0 * s, 109.0 * s],
        size: [(w - 310.0) * s, 2.0 * s]
    };

    let divider = Rect {
        backdrop: [215.0 / 255.0, 205.0 / 255.0, 188.0 / 255.0, 1.0],
        border: Border::new(
            [0.0; 4],
            0.0,
            0.0
        ),
        position: [308.0 * s, 44.0 * s],
        size: [2.0 * s, (h - 75.0) * s]
    };

    // Doc line numbers
    let dln = Rect {
        backdrop: [233.0 / 255.0, 225.0 / 255.0, 213.0 / 255.0, 1.0],
        border: Border::new(
            [0.0; 4],
            0.0,
            0.0
        ),
        position: [371.0 * s, 111.0 * s],
        size: [2.0 * s, (h - 142.0) * s]
    };

    // Filesystem heading
    let fh = Rect {
        backdrop: [233.0 / 255.0, 225.0 / 255.0, 213.0 / 255.0, 1.0],
        border: Border::new(
            [0.0; 4],
            0.0,
            0.0
        ),
        position: [0.0, 109.0 * s],
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
        position: [38.0 * s, 111.0 * s],
        size: [2.0 * s, (h - 142.0) * s]
    };

    let mode = Rect {
        backdrop: [183.0 / 255.0, 60.0 / 255.0, 50.0 / 255.0, 1.0],
        border: Border::new(
            [0.0; 4],
            0.0,
            0.0
        ),
        position: [0.0, (h - 29.0) * s],
        size: [50.0 * s, 29.0 * s]
    };

    let status = Rect {
        backdrop: [235.0 / 255.0, 228.0 / 255.0, 215.0 / 255.0, 1.0],
        border: Border::new(
            [0.0; 4],
            0.0,
            0.0
        ),
        position: [0.0, (h - 29.0) * s],
        size: [w * s, 29.0 * s]
    };

    // Status border
    let sb = Rect {
        backdrop: [215.0 / 255.0, 205.0 / 255.0, 188.0 / 255.0, 1.0],
        border: Border::new(
            [0.0; 4],
            0.0,
            0.0
        ),
        position: [0.0, (h - 31.0) * s],
        size: [w * s, 2.0 * s]
    };

    let title = Rect {
        backdrop: [231.0 / 255.0, 223.0 / 255.0, 208.0 / 255.0, 1.0],
        border: Border::new(
            [0.0; 4],
            0.0,
            0.0
        ),
        position: [0.0, 0.0],
        size: [w * s, 42.0 * s]
    };

    // Title border
    let tb = Rect {
        backdrop: [215.0 / 255.0, 205.0 / 255.0, 188.0 / 255.0, 1.0],
        border: Border::new(
            [0.0; 4],
            0.0,
            0.0
        ),
        position: [0.0, 42.0 * s],
        size: [w * s, 2.0 * s]
    };

    

    rects.write().unwrap().extend([
        title,
        tb,
        status,
        sb,
        divider,
        fh,
        dh,
        fln,
        dln,
        mode
    ]);
    
    let rects = rects.read().unwrap();
    resources.carreau.write().unwrap().prepare(&resources.gpu.queue, &rects);
}
