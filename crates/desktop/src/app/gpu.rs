use wgpu::{Device, DeviceDescriptor, Instance, Queue, RequestAdapterError, RequestAdapterOptions, RequestDeviceError, Surface, SurfaceConfiguration};

pub(super) struct Gpu {
    configuration: Option<SurfaceConfiguration>,
    device: Device,
    queue: Queue,
    surface: Option<Surface<'static>>
}

// Temporary
enum Error {
    RequestAdapter(RequestAdapterError),
    RequestDevice(RequestDeviceError)
}

impl Gpu {
    pub(super) async fn new() -> Result<Self, Error> {
        let instance = Instance::default();

        let adapter = instance.request_adapter(&RequestAdapterOptions::default()).await.map_err(Error::RequestAdapter)?;

        let (device, queue) = adapter.request_device(&DeviceDescriptor::default()).await.map_err(Error::RequestDevice)?;
        
        Ok(Self {
            configuration: None,
            device: device,
            queue: queue,
            surface: None
        })
    }
}