use winit::dpi::PhysicalSize;

#[derive(Clone, Copy, Default)]
pub(crate) struct Layout {
    pub(super) focused: bool,
    pub(super) fw: f32, // Filesystem width
    pub(super) h: f32,
    pub(super) s: f32,
    pub(super) w: f32
}

impl Layout {
    pub(crate) fn new(
        focused: bool,
        fw: f32,
        s: f32,
        size: PhysicalSize<u32>
    ) -> Self {
        Self {
            focused,
            fw,
            h: size.height as f32 / s,
            s,
            w: size.width as f32 / s
        }
    }

    pub(super) fn de(self) -> f32 {
        if self.focused {
            0.0
        }
        else {
            self.fw + 2.0
        }
    }

    pub(crate) fn resize(
        self,
        s: f32,
        size: PhysicalSize<u32>
    ) -> Self {
        Self::new(self.focused, self.fw, s, size)
    }
}
