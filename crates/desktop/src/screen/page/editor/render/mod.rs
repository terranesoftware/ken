mod background;

use background::background;

mod layout;
use layout::layout;

mod rects;

mod text;

use std::sync::{Arc, OnceLock};

use tokio::task::spawn_blocking;
use wgpu::CommandEncoderDescriptor;
use winit::dpi::PhysicalSize;

use crate::{Error, screen::{Resources, page::Editor}};

static LAYOUT: OnceLock<(f32, f32, f32, f32, f32)> = OnceLock::new();

impl Editor {
    pub(crate) async fn prepare(
        &self,
        resources: &Arc<Resources>,
        size: PhysicalSize<u32>
    ) -> Result<(), Error> {
        /* _ = LAYOUT.set(layout(size));

        let mrec = self.rects.clone();
        let mres = resources.clone();
        let rects = spawn_blocking(|| rects::prepare(mrec, mres));

        let mb = self.buffers.clone();
        let mres = resources.clone();
        let text = spawn_blocking(|| text::prepare(mb, mres));

        rects.await?;
        text.await? */

        Ok(())
    }
    
    pub(crate) fn render(&self, resources: &Resources) -> Result<(), Error> {
        let mut encoder = resources.gpu.device.create_command_encoder(&CommandEncoderDescriptor::default());

        {
            let mut rp = background(&mut encoder, &resources.tv);
            
            // rects::render(&resources.carreau.read().unwrap(), &mut rp, resources.tv.texture());

            // text::render(&mut rp, &resources.text.ta, &resources.text.tr, &resources.text.viewport)?;
        }
        
        resources.gpu.queue.submit([encoder.finish()]);

        Ok(())
    }
}