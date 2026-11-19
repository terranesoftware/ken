use std::sync::Arc;

use wgpu::{Device, Extent3d, Surface, SurfaceConfiguration, TextureDescriptor, TextureDimension, TextureUsages, TextureView, TextureViewDescriptor};
use winit::window::Window;

pub(super) fn complete(
    device: Arc<Device>,
    mut sc: SurfaceConfiguration,
    surface: Surface<'static>,
    window: Arc<Window>
) -> (
    (Surface<'static>, SurfaceConfiguration),
    TextureView
) {
    // Get the maximum height of any monitor and maximum width of any monitor
    let (height, width) = window.available_monitors().fold(
        (0, 0),
        |(height, width), monitor| (height.max(monitor.size().height), width.max(monitor.size().width))
    );
    
    // Create a persistent texture for efficiency (retained rendering)
    sc.usage |= TextureUsages::COPY_DST;
    let texture = device.create_texture(&TextureDescriptor {
        dimension: TextureDimension::D2,
        format: sc.format,
        label: None,
        mip_level_count: 1,
        sample_count: 1,
        size: Extent3d {
            depth_or_array_layers: 1,
            height,
            width,
        },
        usage: TextureUsages::COPY_SRC | TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[]
    });
    let tv = texture.create_view(&TextureViewDescriptor::default());
    
    // Configure the surface
    surface.configure(&device, &sc);
    
    ((surface, sc), tv)
}