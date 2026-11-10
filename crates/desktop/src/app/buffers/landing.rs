use glyphon::{Buffer, Metrics};

pub(super) struct Landing {
    name: Buffer,
    new: Buffer,
    nc: Buffer, // New Command
    open: Buffer,
    oc: Buffer, // Open Command
    recent: Buffer,
    re: Buffer, // Recents Empty
    tagline: Buffer
}

impl Landing {
    pub fn new() -> Self {
        Self {
            name: Buffer::new_empty(Metrics::new(40.0, 48.0)),
            new: Buffer::new_empty(Metrics::new(15.0, 22.0)),
            nc: Buffer::new_empty(Metrics::new(12.0, 18.0)),
            open: Buffer::new_empty(Metrics::new(15.0, 22.0)),
            oc: Buffer::new_empty(Metrics::new(12.0, 18.0)),
            recent: Buffer::new_empty(Metrics::new(11.0, 16.0)),
            re: Buffer::new_empty(Metrics::new(14.0, 20.0)),
            tagline: Buffer::new_empty(Metrics::new(13.0, 20.0))
        }
    }
}