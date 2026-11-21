mod gpu;
pub(super) use gpu::Gpu;

mod text;
pub(super) use text::Text;

use std::sync::RwLock;

use carreau::Carreau;
use wgpu::TextureView;

pub(crate) struct Resources {
    pub(crate) carreau: RwLock<Carreau>,
    pub(crate) gpu: Gpu,
    pub(crate) text: Text,
    pub(crate) tv: TextureView
}