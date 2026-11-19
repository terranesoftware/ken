use std::sync::Arc;

use wgpu::{Device, Queue};

pub(crate) struct Gpu {
    pub(crate) device: Arc<Device>,
    pub(crate) queue: Arc<Queue>,
}
