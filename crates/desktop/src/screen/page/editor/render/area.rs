use glyphon::{Buffer, Color, TextArea, TextBounds};

pub(crate) fn area<'a>(
    buffer: &'a Buffer,
    left: f32,
    top: f32,
    right: f32,
    s: f32,
    ink: Color
) -> TextArea<'a> {
    TextArea {
        bounds: TextBounds {
            right: (right * s) as i32,
            ..TextBounds::default()
        },
        buffer,
        custom_glyphs: &[],
        default_color: ink,
        left: left * s,
        scale: s,
        top: top * s
    }
}