use winit::dpi::PhysicalSize;

#[derive(Clone, Copy)]
pub(crate) struct Layout {
    pub(super) h: f32,
    pub(super) s: f32,
    pub(super) w: f32
}

impl Layout {
    pub(crate) fn new(
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
