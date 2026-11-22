mod buffers;
pub(super) use buffers::Buffers;

mod layout;
use ken::Ken;
pub(crate) use layout::Layout;

mod render;

use std::{path::PathBuf, sync::{Arc, RwLock}};

use carreau::Rect;
use wgpu::CommandEncoderDescriptor;
use winit::{dpi::{PhysicalPosition, PhysicalSize}, window::{CursorIcon, Window}};

use crate::{Error, screen::{Page, Resources, page::{Destination, PageMethods, Palette, background, editor::render::{rects, text}}}};

pub(crate) struct Editor {
    pub(crate) buffers: Arc<RwLock<Buffers>>,
    pub(crate) focused: bool,
    pub(crate) fw: RwLock<f32>, // Filesystem width
    pub(crate) ken: Arc<RwLock<Ken>>,
    pub(crate) rects: Arc<RwLock<Vec<Rect>>>
}

impl Page {
    pub(crate) async fn editor(
        ken: Arc<RwLock<Ken>>,
        palette: &Palette,
        path: Option<PathBuf>,
        resources: &Arc<Resources>,
        scale: f32,
        size: PhysicalSize<u32>
    ) -> Result<Self, Error> {
        let focused = if let Some(path) = path {
            let mut ken = ken.write().unwrap();
            ken.active = path.clone();
            ken.directories.insert(path)?;
            
            false
        }
        else {
            true
        };
        
        let editor = Arc::new(Editor {
            buffers: Arc::new(RwLock::new(Buffers::new(&ken, palette))),
            focused,
            fw: RwLock::new(308.0),
            ken,
            rects: Arc::new(RwLock::new(Vec::new()))
        });
        
        editor.prepare(Layout::new(308.0, scale, size), *palette, resources).await?;
        
        editor.render(palette, resources)?;
        
        Ok(Self::Editor(editor))
    }
}

impl PageMethods for Arc<Editor> {
    fn clicked(
        &self,
        position: PhysicalPosition<f64>,
        window: &Window
    ) -> Option<Destination> {
        None
    }
    
    fn moved(
        &self,
        position: PhysicalPosition<f64>,
        resources: &Resources
    ) -> CursorIcon {
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