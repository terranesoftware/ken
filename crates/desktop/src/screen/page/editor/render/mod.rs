mod rects;

// mod text;

use std::sync::Arc;

use tokio::task::spawn_blocking;
use wgpu::CommandEncoderDescriptor;

use crate::{Error, screen::{Resources, page::{Editor, background, editor::Layout}}};

impl Editor {
    pub(super) async fn prepare(
        &self,
        layout: Layout,
        resources: &Arc<Resources>
    ) -> Result<(), Error> {
        let mrec = self.rects.clone();
        let mres = resources.clone();
        spawn_blocking(move || rects::prepare(layout, mrec, mres)).await?;

        Ok(())
    }
    
    pub(crate) fn render(
        &self,
        resources: &Arc<Resources>
    ) -> Result<(), Error> {
        let mut encoder = resources.gpu.device.create_command_encoder(&CommandEncoderDescriptor::default());

        {
            let mut rp = background(&mut encoder, &resources.tv);
            
            rects::render(&resources.carreau.read().unwrap(), &mut rp, resources.tv.texture());

            // text::render(&mut rp, &resources.text.ta, &resources.text.tr, &resources.text.viewport)?;
        }
        
        resources.gpu.queue.submit([encoder.finish()]);

        Ok(())
    }
}