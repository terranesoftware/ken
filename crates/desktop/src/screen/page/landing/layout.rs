use winit::dpi::PhysicalSize;

// All fields are logical pixels; `s` converts them to physical
#[derive(Clone, Copy)]
pub(super) struct Layout {
    pub(super) cw: f32,
    pub(super) left: f32,
    pub(super) right: f32,
    pub(super) s: f32,
    pub(super) top: f32,
    pub(super) w: f32
}

impl Layout {
    pub(super) fn new(
        s: f32,
        size: PhysicalSize<u32>
    ) -> Self {
        let h = size.height as f32 / s;
        let w = size.width as f32 / s;
        
        let cw = w - 1000.0;
        let left = (w - cw) / 2.0;
        let right = cw + left;
        
        let top = (h - 500.0) / 2.0;
        
        Self {
            cw,
            left,
            right,
            s,
            top,
            w
        }
    }
}