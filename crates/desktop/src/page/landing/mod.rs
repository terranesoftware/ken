mod buffers;
pub(super) use buffers::Buffers;

mod render;

use std::sync::{Arc, RwLock};

use carreau::Rect;
use wgpu::Surface;
use winit::{dpi::PhysicalPosition, window::Window};

use crate::{Error, Page, page::resources::Resources, resources::{Gpu, Text}};

pub(crate) struct Landing {
    pub(crate) buffers: Arc<RwLock<Buffers>>,
    pub(crate) rects: Arc<RwLock<Vec<Rect>>>,
    pub(crate) resources: Resources,
    pub(crate) window: Arc<Window>
}

impl Page {
    pub(crate) async fn landing(
        gpu: Arc<Gpu>,
        surface: Surface<'static>,
        text: Arc<Text>,
        window: Arc<Window>
    ) -> Result<Self, Error> {
        let resources = Resources::new(gpu, surface, text, window.clone()).await?;
        
        let landing = Landing {
            buffers: Arc::new(RwLock::new(Buffers::new())),
            rects: Arc::new(RwLock::new(Vec::new())),
            resources,
            window
        };
        landing.prepare().await?;
        landing.render()?;
        
        Ok(Self::Landing(landing))
    }
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