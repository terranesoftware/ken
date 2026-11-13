mod background;
use background::background;

mod text;

use tokio::task::spawn_blocking;
use wgpu::CommandEncoderDescriptor;

use crate::{Error, page::Landing};

impl Landing {
    pub(crate) async fn render(&self) -> Result<(), Error> {
        let gpu = &self.resources.gpu;
        let text = &self.resources.text;
        
        let device = &gpu.device;
        let queue = &gpu.queue;
        let ta = &text.ta;
        let tr = &text.tr;
        let view = &gpu.view;
        let viewport = &text.viewport;

        let mb = self.buffers.clone();
        let mg = gpu.clone();
        let mt = text.clone();
        let mw = self.window.clone();
        let text = spawn_blocking(|| text::prepare(mb, mg, mt, mw));
        
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());

        {
            let mut rp = background(&mut encoder, view);
            
            // rects()
            
            text.await??;
            text::render(&mut rp, ta, tr, viewport)?;
        }
        
        queue.submit([encoder.finish()]);

        Ok(())
    }
}