use std::sync::Arc;

use wgpu::{Adapter, Backends, Device, DeviceDescriptor, Instance, InstanceDescriptor, Queue, RequestAdapterOptions};

use crate::Error;

pub(crate) struct Gpu {
    pub(crate) adapter: Adapter,
    pub(crate) device: Arc<Device>,
    pub(crate) instance: Instance,
    pub(crate) queue: Arc<Queue>
}

impl Gpu {
    pub(super) async fn new(rao: Option<RequestAdapterOptions<'_, '_>>) -> Result<Self, Error> {
        #[cfg(target_os = "macos")]
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::METAL,
            ..InstanceDescriptor::new_without_display_handle()
        });
        #[cfg(target_os = "windows")]
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
            device: Arc::new(device),
            instance,
            queue: Arc::new(queue)
        })
    }
}