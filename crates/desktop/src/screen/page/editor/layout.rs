use winit::dpi::PhysicalSize;

#[derive(Clone, Copy)]
pub(crate) struct Layout {
    pub(super) fw: f32, // Filesystem width
    pub(super) h: f32,
    pub(super) s: f32,
    pub(super) w: f32
}

impl Layout {
    pub(crate) fn new(
        fw: f32,
        s: f32,
        size: PhysicalSize<u32>
    ) -> Self {
        Self {
            fw,
            h: size.height as f32 / s,
            s,
            w: size.width as f32 / s
        }
    }
}
