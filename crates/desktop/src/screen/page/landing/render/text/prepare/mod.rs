mod layout;

mod shape;
use shape::shape;

mod uv;
use uv::uv;

use std::sync::Arc;

use crate::{Error, screen::{Resources, page::{Palette, landing::{Landing, Layout}}}};

pub(crate) fn prepare(
    landing: &Landing,
    layout: Layout,
    palette: Palette,
    resources: Arc<Resources>
) -> Result<(), Error> {
    let gpu = &resources.gpu;
    let text = &resources.text;

    // Shape the buffers
    shape(
        &mut text.fs.write().unwrap(),
        landing
    );
    
    // Update the viewport
    uv(
        &gpu.queue,
        resources.tv.texture(),
        &mut text.viewport.write().unwrap()
    );
    
    // Create the layout of the text
    layout::layout(
        &gpu.device,
        &mut text.fs.write().unwrap(),
        landing,
        layout,
        palette,
        &gpu.queue,
        &mut text.sc.write().unwrap(),
        &mut text.ta.write().unwrap(),
        &mut text.tr.write().unwrap(),
        &text.viewport.read().unwrap()
    )
}