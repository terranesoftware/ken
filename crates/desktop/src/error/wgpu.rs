use wgpu::{CreateSurfaceError, RequestAdapterError, RequestDeviceError, rwh::HandleError};

use crate::Error;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum WgpuError {
    CreateSurface(CreateSurfaceError),
    Handle(HandleError),
    RequestAdapter(RequestAdapterError),
    RequestDevice(RequestDeviceError)
}

impl From<CreateSurfaceError> for Error {
    fn from(value: CreateSurfaceError) -> Self {
        Error::Wgpu(WgpuError::CreateSurface(value))
    }
}

impl From<HandleError> for Error {
    fn from(value: HandleError) -> Self {
        Error::Wgpu(WgpuError::Handle(value))
    }
}

impl From<RequestAdapterError> for Error {
    fn from(value: RequestAdapterError) -> Self {
        Error::Wgpu(WgpuError::RequestAdapter(value))
    }
}

impl From<RequestDeviceError> for Error {
    fn from(value: RequestDeviceError) -> Self {
        Error::Wgpu(WgpuError::RequestDevice(value))
    }
}