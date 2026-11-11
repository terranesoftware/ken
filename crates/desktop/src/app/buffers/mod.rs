mod landing;
pub(super) use landing::Landing;

pub(super) struct Buffers {
    pub(super) landing: Landing
}

impl Buffers {
    pub(super) fn new() -> Self {
        Self {
            landing: Landing::new()
        }
    }
}