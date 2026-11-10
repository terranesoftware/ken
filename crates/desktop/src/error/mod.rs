mod wgpu;
use wgpu::WgpuError;

mod winit;
use winit::WinitError;

#[allow(dead_code)]
#[derive(Debug)]
pub(super) enum Error {
    Wgpu(WgpuError),
    Winit(WinitError)
}