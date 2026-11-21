mod layout;

mod shape;
use shape::shape;

mod uv;
use uv::uv;

use std::sync::{Arc, RwLock};

use crate::{Error, screen::{Resources, page::editor::{Buffers, Layout}}};

pub(crate) fn prepare(
    buffers: Arc<RwLock<Buffers>>,
    layout: Layout,
    resources: Arc<Resources>
) -> Result<(), Error> {
    let gpu = &resources.gpu;
    let text = &resources.text;

    // Shape the buffers
    shape(
        &mut buffers.write().unwrap(),
        &mut text.fs.write().unwrap()
    );
    
    // Update the viewport
    uv(
        &gpu.queue,
        resources.tv.texture(),
        &mut text.viewport.write().unwrap()
    );
    
    // Create the layout of the text
    layout::layout(
        &buffers.read().unwrap(),
        &gpu.device,
        &mut text.fs.write().unwrap(),
        layout,
        &gpu.queue,
        &mut text.sc.write().unwrap(),
        &mut text.ta.write().unwrap(),
        &mut text.tr.write().unwrap(),
        &text.viewport.read().unwrap()
    )
}