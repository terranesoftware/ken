mod landing;
use landing::Landing;

pub(super) struct Buffers {
    landing: Landing
}

impl Buffers {
    pub(super) fn new() -> Self {
        Self {
            landing: Landing::new()
        }
    }
}