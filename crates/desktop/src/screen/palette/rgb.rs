use glyphon::Color;

#[derive(Clone, Copy)]
pub(crate) struct Rgb(pub(super) u8, pub(super) u8, pub(super) u8);

impl Rgb {
    // For carreau 
    pub(crate) fn rect(self) -> [f32; 4] {
        [self.0 as f32 / 255.0, self.1 as f32 / 255.0, self.2 as f32 / 255.0, 1.0]
    }

    // For glyphon
    pub(crate) fn text(self) -> Color {
        Color::rgb(self.0, self.1, self.2)
    }
}