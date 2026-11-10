use std::sync::Arc;

use wgpu::{Adapter, Backends, Device, DeviceDescriptor, Instance, InstanceDescriptor, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration};

use crate::Error;

pub(crate) struct Gpu {
    pub(super) adapter: Adapter,
    pub(super) device: Device,
    pub(super) instance: Instance,
    pub(super) queue: Queue,
    pub(super) sc: Option<SurfaceConfiguration>,
    pub(super) surface: Option<Arc<Surface<'static>>>
}

impl Gpu {
    pub(super) async fn new(rao: Option<RequestAdapterOptions<'_, '_>>) -> Result<Self, Error> {
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::GL,
            ..InstanceDescriptor::new_without_display_handle()
        });

        // Speculative, not confirmed whether it works with the window/surface
        let adapter = if let Some(rao) = rao {
            instance.request_adapter(&rao).await?
        }
        else {
            instance.request_adapter(&RequestAdapterOptions::default()).await?
        };

        let (device, queue) = adapter.request_device(&DeviceDescriptor::default()).await?;
        
        Ok(Self {
            adapter,
            device,
            instance,
            queue,
            sc: None,
            surface: None
        })
    }
}