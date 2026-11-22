mod glyphon;
use glyphon::GlyphonError;

mod tokio;
use tokio::TokioError;

mod wgpu;
use wgpu::WgpuError;

mod winit;
use winit::WinitError;

use ken::Error as KenError;

#[allow(dead_code)]
#[derive(Debug)]
pub(super) enum Error {
    Glyphon(GlyphonError),
    Ken(KenError),
    Tokio(TokioError),
    Wgpu(WgpuError),
    Winit(WinitError)
}

impl From<KenError> for Error {
    fn from(value: KenError) -> Self {
        Error::Ken(value)
    }
}