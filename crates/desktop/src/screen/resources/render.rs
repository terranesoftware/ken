use wgpu::CommandEncoderDescriptor;

use crate::{Error, screen::{Palette, Resources, resources::background}};

impl Resources {
    pub(crate) fn render(
        &self,
        palette: &Palette
    ) -> Result<(), Error> {
        let mut encoder = self.gpu.device.create_command_encoder(&CommandEncoderDescriptor::default());
        
        {
            let mut rp = background(&mut encoder, &self.tv, palette.paper);
            
            let texture = self.tv.texture();
            self.carreau.read().unwrap().render(&mut rp, [texture.width() as f32, texture.height() as f32]);
            
            let ta = self.text.ta.read().unwrap();
            let tr = self.text.tr.read().unwrap();
            let viewport = self.text.viewport.read().unwrap();

            tr.render(&ta, &viewport, &mut rp)?;
        }
        
        self.gpu.queue.submit([encoder.finish()]);
        
        Ok(())
    }
}
