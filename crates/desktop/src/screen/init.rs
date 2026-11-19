use std::sync::Arc;

use wgpu::{Surface, SurfaceConfiguration};
use winit::window::Window;

use crate::{Error, resources::Gpu};

pub(super) fn init(
    gpu: Arc<Gpu>,
    surface: Surface<'static>,
    window: Arc<Window>
) -> Result<(
    SurfaceConfiguration,
    Surface<'static>
), Error> {
    let adapter = &gpu.adapter;
    
    // TODO:
    // If the surface isn't supported, retry, guaranteeing it'll work (or if no GPUs are compatible, it'll just error)
    if !adapter.is_surface_supported(&surface) {
        todo!();
    }
    
    // Create a default config
    let size = window.inner_size();
    let sc = surface.get_default_config(adapter, size.width, size.height).unwrap();
    
    Ok((sc, surface))
}