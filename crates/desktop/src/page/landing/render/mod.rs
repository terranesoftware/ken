mod background;

use background::background;

mod layout;
use layout::layout;

mod rects;

mod text;

use std::sync::OnceLock;

use tokio::task::spawn_blocking;
use wgpu::CommandEncoderDescriptor;

use crate::{Error, page::Landing};

static LAYOUT: OnceLock<(f32, f32, f32, f32, f32)> = OnceLock::new();

impl Landing {
    pub(crate) async fn prepare(&self) -> Result<(), Error> {
        _ = LAYOUT.set(layout(self.window.inner_size()));

        let mc = self.resources.carreau.clone();
        let mq = self.resources.gpu.queue.clone();
        let mr = self.rects.clone();
        let rects = spawn_blocking(|| rects::prepare(mc, mq, mr));

        let mb = self.buffers.clone();
        let mg = self.resources.gpu.clone();
        let mt = self.resources.text.clone();
        let text = spawn_blocking(|| text::prepare(mb, mg, mt));

        rects.await?;
        text.await?
    }
    
    pub(crate) fn render(&self) -> Result<(), Error> {
        let mut encoder = self.resources.gpu.device.create_command_encoder(&CommandEncoderDescriptor::default());

        {
            let mut rp = background(&mut encoder, &self.resources.gpu.view);
            
            rects::render(&self.resources.carreau.read().unwrap(), &mut rp, &self.resources.gpu.texture);

            text::render(&mut rp, &self.resources.text.ta, &self.resources.text.tr, &self.resources.text.viewport)?;
        }
        
        self.resources.gpu.queue.submit([encoder.finish()]);

        Ok(())
    }
}