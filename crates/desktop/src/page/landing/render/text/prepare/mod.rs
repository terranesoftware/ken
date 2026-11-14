mod layout;
use layout::layout;

mod shape;
use shape::shape;

mod uv;
use uv::uv;

use std::sync::{Arc, RwLock};

use winit::window::Window;

use crate::{Error, page::{Buffers, resources::{Gpu, Text}}};

pub(crate) fn prepare(
    buffers: Arc<RwLock<Buffers>>,
    gpu: Arc<Gpu>,
    text: Arc<Text>,
    window: Arc<Window>
) -> Result<(), Error> {
    // Shape the buffers
    shape(
        &mut buffers.write().unwrap(),
        &mut text.fs.write().unwrap()
    );
    
    // Update the viewport
    uv(
        &gpu.queue,
        &gpu.texture,
        &mut text.viewport.write().unwrap()
    );
    
    // Create the layout of the text
    layout(
        &buffers.read().unwrap(),
        &gpu.device,
        &mut text.fs.write().unwrap(),
        &gpu.queue,
        &mut text.sc.write().unwrap(),
        &mut text.ta.write().unwrap(),
        &mut text.tr.write().unwrap(),
        &text.viewport.read().unwrap(),
        window.inner_size()
    )
}