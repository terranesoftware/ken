mod buffers;
pub(super) use buffers::Buffers;

mod layout;
pub(crate) use layout::Layout;
use rfd::FileDialog;

mod render;

use std::sync::{Arc, RwLock};

use carreau::Rect;
use wgpu::CommandEncoderDescriptor;
use winit::{dpi::{PhysicalPosition, PhysicalSize}, window::{CursorIcon, Window}};

use crate::{Error, screen::{Page, Resources, page::{Destination, PageMethods, Palette, background, landing::render::{rects, text}}}};

pub(crate) struct Landing {
    pub(crate) buffers: Arc<RwLock<Buffers>>,
    pub(crate) rects: Arc<RwLock<Vec<Rect>>>
}

impl Page {
    pub(crate) async fn landing(
        palette: &Palette,
        resources: &Arc<Resources>,
        scale: f32,
        size: PhysicalSize<u32>
    ) -> Result<Self, Error> {
        let landing = Arc::new(Landing {
            buffers: Arc::new(RwLock::new(Buffers::new(palette))),
            rects: Arc::new(RwLock::new(Vec::new()))
        });

        landing.prepare(Layout::new(scale, size), *palette, resources).await?;
        landing.render(palette, resources)?;
        
        Ok(Self::Landing(landing))
    }
}

impl PageMethods for Arc<Landing> {
    fn clicked(
        &self,
        position: PhysicalPosition<f64>,
        window: &Window
    ) -> Option<Destination> {
        let point = [position.x as f32, position.y as f32];
        let rects = self.rects.read().unwrap();

        for (index, rect) in rects.iter().enumerate() {
            if index == 2 {
                continue;
            }

            if rect.hit(point) {
                // New
                if index == 0 {
                    return Some(Destination::Editor(None))
                }

                // Open
                if index == 1 {
                    match FileDialog::new().set_parent(window).pick_folder() {
                        Some(path) => return Some(Destination::Editor(Some(path))),
                        None => return None
                    }
                }
            }
        }

        None
    }
    
    fn moved(
        &self,
        position: PhysicalPosition<f64>,
        resources: &Resources
    ) -> CursorIcon {
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

        CursorIcon::Default
    }

    fn render(
        &self,
        palette: &Palette,
        resources: &Arc<Resources>
    ) -> Result<(), Error> {
        let mut encoder = resources.gpu.device.create_command_encoder(&CommandEncoderDescriptor::default());
        
        {
            let mut rp = background(&mut encoder, &resources.tv, palette.paper);
            
            rects::render(&resources.carreau.read().unwrap(), &mut rp, resources.tv.texture());
            
            text::render(&mut rp, &resources.text.ta, &resources.text.tr, &resources.text.viewport)?;
        }
        
        resources.gpu.queue.submit([encoder.finish()]);
        
        Ok(())
    }
}