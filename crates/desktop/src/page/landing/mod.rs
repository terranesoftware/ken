mod buffers;
pub(super) use buffers::Buffers;

mod render;

use std::sync::{Arc, RwLock};

use carreau::Rect;
use winit::{dpi::PhysicalPosition, window::Window};

use crate::page::resources::Resources;

pub(crate) struct Landing {
    pub(crate) buffers: Arc<RwLock<Buffers>>,
    pub(crate) rects: Arc<RwLock<Vec<Rect>>>,
    pub(crate) resources: Resources,
    pub(crate) window: Arc<Window>
}

impl Landing {
    pub(super) fn moved(
        &self,
        position: PhysicalPosition<f64>
    ) {
        let point = [position.x as f32, position.y as f32];
        let mut rects = self.rects.write().unwrap();

        for (index, rect) in rects.iter_mut().enumerate() {
            if index == 2 {
                continue;
            }

            rect.backdrop[3] = if rect.hit(point) {
                1.0
            }
            else {
                0.0
            };
        }

        self.resources.carreau.write().unwrap().prepare(&self.resources.gpu.queue, &rects);
    }
}