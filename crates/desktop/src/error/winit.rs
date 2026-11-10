use winit::error::EventLoopError;

use crate::Error;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum WinitError {
    EventLoop(EventLoopError)
}

impl From<EventLoopError> for Error {
    fn from(value: EventLoopError) -> Self {
        Error::Winit(WinitError::EventLoop(value))
    }
}