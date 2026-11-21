mod buffers;
pub(super) use buffers::Buffers;

mod layout;
pub(crate) use layout::Layout;

mod render;

use std::sync::{Arc, RwLock};

use carreau::Rect;
use wgpu::CommandEncoderDescriptor;
use winit::dpi::{PhysicalPosition, PhysicalSize};

use crate::{Error, screen::{Page, Resources, page::{Destination, PageMethods, Palette, background, editor::render::{rects, text}}}};

pub(crate) struct Editor {
    pub(crate) buffers: Arc<RwLock<Buffers>>,
    pub(crate) rects: Arc<RwLock<Vec<Rect>>>
}

impl Page {
    pub(crate) async fn editor(
        palette: &Palette,
        resources: &Arc<Resources>,
        scale: f32,
        size: PhysicalSize<u32>
    ) -> Result<Self, Error> {
        let editor = Arc::new(Editor {
            buffers: Arc::new(RwLock::new(Buffers::new(palette))),
            rects: Arc::new(RwLock::new(Vec::new()))
        });
        editor.prepare(Layout::new(scale, size), *palette, resources).await?;
        editor.render(palette, resources)?;
        
        Ok(Self::Editor(editor))
    }
}

impl PageMethods for Arc<Editor> {
    fn clicked(
        &self,
        position: PhysicalPosition<f64>
    ) -> Option<Destination> {
        None
    }
    
    fn moved(
        &self,
        position: PhysicalPosition<f64>,
        resources: &Resources
    ) {
        
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