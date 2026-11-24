mod ephemeral;
pub use ephemeral::Ephemeral;

mod persistent;
pub use persistent::Persistent;

use std::path::PathBuf;

use crate::buffers::files::ephemeral::EphemeralKey;

pub enum File {
    Ephemeral(EphemeralKey),
    Persistent(PathBuf)
}

pub struct Files {
    pub active: Option<File>,
    pub ephemeral: Ephemeral,
    pub persistent: Persistent
}

impl Files {
    pub fn new() -> Self {
        Self {
            active: None,
            ephemeral: Ephemeral::new(),
            persistent: Persistent::new()
        }
    }
}