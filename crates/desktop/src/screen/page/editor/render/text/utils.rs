use glyphon::{Buffer, Color, TextArea, TextBounds};

pub(super) fn area<'a>(
    buffer: &'a Buffer,
    left: f32,
    top: f32,
    s: f32,
    ink: Color
) -> TextArea<'a> {
    TextArea {
        bounds: TextBounds::default(),
        buffer,
        custom_glyphs: &[],
        default_color: ink,
        left: left * s,
        scale: s,
        top: top * s
    }
}