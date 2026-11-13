use tokio::task::JoinError;

use crate::Error;

#[derive(Debug)]
pub(crate) enum TokioError {
    Join(JoinError)
}

impl From<JoinError> for Error {
    fn from(value: JoinError) -> Self {
        Error::Tokio(TokioError::Join(value))
    }
}