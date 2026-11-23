mod buffers;
pub(super) use buffers::Buffers;

mod hits;
use hits::Hits;

mod layout;
pub(crate) use layout::Layout;

mod render;

use std::{path::PathBuf, sync::{Arc, RwLock}};

use carreau::Rect;
use ken::Ken;
use wgpu::CommandEncoderDescriptor;
use winit::{dpi::{PhysicalPosition, PhysicalSize}, window::{CursorIcon, Window}};

use crate::{Error, screen::{Page, Resources, page::{Destination, PageMethods, Palette, background, editor::render::{rects, text}}}};

pub(crate) struct Editor {
    pub(crate) buffers: Arc<RwLock<Buffers>>,
    pub(crate) hits: Arc<RwLock<Hits>>,
    pub(crate) ken: Arc<RwLock<Ken>>,
    pub(crate) layout: RwLock<Layout>,
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
            hits: Arc::new(RwLock::new(Hits::default())),
            ken,
            layout: RwLock::new(Layout::default()),
            rects: Arc::new(RwLock::new(Vec::new()))
        });
        
        editor.prepare(Layout::new(focused, 308.0, scale, size), *palette, resources).await?;
        
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
        _resources: &Resources
    ) -> CursorIcon {
        let hits = self.hits.read().unwrap();
        let point = [position.x as f32, position.y as f32];

        // Checked first since it overlaps the filesystem hit rect, gives the "simulation" of a higher z-level
        if let Some(dh) = hits.divider && dh.hit(point) {
            CursorIcon::ColResize
        }
        // Flatten removes any that are None
        else if [hits.file, hits.filesystem].iter().flatten().any(|rect| rect.hit(point)) {
            CursorIcon::Text
        }
        else {
            CursorIcon::Default
        }
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