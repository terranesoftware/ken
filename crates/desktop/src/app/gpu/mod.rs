mod text;
pub(crate) use text::Text;

use winit::event_loop::EventLoopProxy;

use std::sync::Arc;

use tokio::task::spawn_blocking;
use wgpu::{Adapter, Backends, Device, DeviceDescriptor, Instance, InstanceDescriptor, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, Texture, TextureView};

use crate::{Error, UserEvent};

pub(crate) struct Gpu {
    pub(super) adapter: Adapter,
    pub(super) device: Arc<Device>,
    pub(super) instance: Instance,
    pub(super) queue: Arc<Queue>,
    pub(super) sc: Option<SurfaceConfiguration>,
    pub(super) surface: Option<Arc<Surface<'static>>>,
    pub(super) text: Option<Text>,
    pub(super) texture: Option<Texture>,
    pub(super) view: Option<TextureView>
}

impl Gpu {
    pub(super) async fn new(
        elp: Arc<EventLoopProxy<UserEvent>>,
        rao: Option<RequestAdapterOptions<'_, '_>>
    ) -> Result<Self, Error> {
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
        let (device, queue) = (Arc::new(device), Arc::new(queue));

        // Benchmark this later, see if spawning is worth it
        let moved = device.clone();
        spawn_blocking(move || {
            let text = Text::new(moved);
            _ = elp.send_event(UserEvent::Text(text));
        });

        Ok(Self {
            adapter,
            device,
            instance,
            queue,
            sc: None,
            surface: None,
            text: None,
            texture: None,
            view: None
        })
    }
}