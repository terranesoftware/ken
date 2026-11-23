use std::sync::Arc;

use wgpu::CommandEncoderDescriptor;
use winit::{dpi::PhysicalPosition, window::{CursorIcon, Window}};

use crate::{Error, screen::{Destination, PageMethods, Palette, Resources, page::{Editor, background, editor::render::{rects, text}}}};

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
        _palette: &Palette,
        position: PhysicalPosition<f64>,
        _resources: &Resources
    ) -> CursorIcon {
        let layout = *self.layout.read().unwrap();
        let point = [position.x as f32, position.y as f32];

        // Checked first since it overlaps the filesystem hit rect, gives the "simulation" of a higher z-level
        if !layout.focused && self.divider.hit(layout, point) {
            CursorIcon::ColResize
        }
        // The filesystem pane is absent if focused
        else if self.file.read().unwrap().hit(layout, point) || (!layout.focused && self.filesystem.read().unwrap().hit(layout, point)) {
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