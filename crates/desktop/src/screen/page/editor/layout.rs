use winit::dpi::PhysicalSize;

// All fields are logical pixels; `s` converts them to physical
#[derive(Clone, Copy)]
pub(super) struct Layout {
    pub(super) h: f32,
    pub(super) s: f32,
    pub(super) w: f32
}

impl Layout {
    pub(super) fn new(
        s: f32,
        size: PhysicalSize<u32>
    ) -> Self {
        Self {
            h: size.height as f32 / s,
            s,
            w: size.width as f32 / s
        }
    }
}
