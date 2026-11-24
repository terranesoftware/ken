use glyphon::{FontSystem, Resolution, TextArea};

use crate::{Error, screen::Resources};

impl Resources {
    pub(crate) fn areas<'a>(
        &self,
        areas: impl IntoIterator<Item = TextArea<'a>>,
        fs: &mut FontSystem
    ) -> Result<(), Error> {
        let texture = self.tv.texture();
        self.text.viewport.write().unwrap().update(&self.gpu.queue, Resolution {
            height: texture.height(),
            width: texture.width()
        });

        let ta = &mut self.text.ta.write().unwrap();
        let mut tr = self.text.tr.write().unwrap();
        let viewport = self.text.viewport.read().unwrap();

        tr.prepare(&self.gpu.device, &self.gpu.queue, fs, ta, &viewport, areas, &mut self.text.sc.write().unwrap())?;

        Ok(())
    }
}
