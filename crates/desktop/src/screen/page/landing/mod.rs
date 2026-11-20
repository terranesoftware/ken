mod buffers;
pub(super) use buffers::Buffers;

mod render;

use std::sync::{Arc, RwLock};

use carreau::Rect;
use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::{Error, screen::{Page, Resources, page::Destination}};

pub(crate) struct Landing {
    pub(crate) buffers: Arc<RwLock<Buffers>>,
    pub(crate) rects: Arc<RwLock<Vec<Rect>>>
}

impl Page {
    pub(crate) async fn landing(
        resources: &Arc<Resources>,
        size: PhysicalSize<u32>
    ) -> Result<Self, Error> {
        let landing = Landing {
            buffers: Arc::new(RwLock::new(Buffers::new())),
            rects: Arc::new(RwLock::new(Vec::new()))
        };
        landing.prepare(resources, size).await?;
        landing.render(resources)?;
        
        Ok(Self::Landing(landing))
    }
}

impl Landing {
    pub(super) fn clicked(
        &self,
        position: PhysicalPosition<f64>
    ) -> Option<Destination> {
        let point = [position.x as f32, position.y as f32];
        let rects = self.rects.read().unwrap();

        for (index, rect) in rects.iter().enumerate() {
            if index == 2 {
                continue;
            }

            if rect.hit(point) {
                return Some(Destination::Editor);
            }
        }

        None
    }
    
    pub(super) fn moved(
        &self,
        position: PhysicalPosition<f64>,
        resources: &Resources
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

        resources.carreau.write().unwrap().prepare(&resources.gpu.queue, &rects);
    }
}