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
    pub(crate) async fn render(&self) -> Result<(), Error> {
        let carreau = &self.resources.carreau;
        let gpu = &self.resources.gpu;
        let text = &self.resources.text;
        
        let device = &gpu.device;
        let queue = &gpu.queue;
        let ta = &text.ta;
        let texture = &gpu.texture;
        let tr = &text.tr;
        let view = &gpu.view;
        let viewport = &text.viewport;

        LAYOUT.set(layout(self.window.inner_size())).unwrap();

        let mc = carreau.clone();
        let mq = queue.clone();
        let rects = spawn_blocking(|| rects::prepare(mc, mq));

        let mb = self.buffers.clone();
        let mg = gpu.clone();
        let mt = text.clone();
        let text = spawn_blocking(|| text::prepare(mb, mg, mt));
        
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());

        {
            let mut rp = background(&mut encoder, view);
            
            rects.await?;
            rects::render(&carreau.read().unwrap(), &mut rp, texture);

            text.await??;
            text::render(&mut rp, ta, tr, viewport)?;
        }
        
        queue.submit([encoder.finish()]);

        Ok(())
    }
}