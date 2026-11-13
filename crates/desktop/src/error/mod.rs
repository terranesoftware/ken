mod glyphon;
use glyphon::GlyphonError;

mod tokio;
use tokio::TokioError;

mod wgpu;
use wgpu::WgpuError;

mod winit;
use winit::WinitError;

#[allow(dead_code)]
#[derive(Debug)]
pub(super) enum Error {
    Glyphon(GlyphonError),
    Tokio(TokioError),
    Wgpu(WgpuError),
    Winit(WinitError)
}