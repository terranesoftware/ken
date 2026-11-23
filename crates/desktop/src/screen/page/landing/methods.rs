use std::sync::Arc;

use rfd::FileDialog;
use wgpu::CommandEncoderDescriptor;
use winit::{dpi::PhysicalPosition, window::{CursorIcon, Window}};

use crate::{Error, screen::{Destination, PageMethods, Palette, Resources, landing::render::{rects, text}, page::{Landing, background}}};

impl PageMethods for Arc<Landing> {
    fn clicked(
        &self,
        position: PhysicalPosition<f64>,
        window: &Window
    ) -> Option<Destination> {
        let layout = *self.layout.read().unwrap();
        let point = [position.x as f32, position.y as f32];

        if self.new.read().unwrap().hit(layout, point) {
            return Some(Destination::Editor(None));
        }
        else if self.open.read().unwrap().hit(layout, point) {
            return FileDialog::new().set_parent(window).pick_folder().map(|path| Destination::Editor(Some(path)));
        }
        else {
            None
        }
    }
    
    fn moved(
        &self,
        palette: &Palette,
        position: PhysicalPosition<f64>,
        resources: &Resources
    ) -> CursorIcon {
        let layout = *self.layout.read().unwrap();
        let point = [position.x as f32, position.y as f32];

        self.new.write().unwrap().hover(layout, point);
        self.open.write().unwrap().hover(layout, point);

        rects::prepare(self, layout, *palette, resources);

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