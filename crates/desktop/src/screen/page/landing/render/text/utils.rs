use glyphon::{Buffer, Color, TextArea, TextBounds};

pub(crate) fn area<'a>(
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

pub(crate) fn center(
    buffer: &Buffer,
    ww: f32 // Window Width
) -> f32 {
    (ww - width(buffer)) / 2.0
}

// Gets the width of the buffer by finding the line with the maximum width
pub(crate) fn width(buffer: &Buffer) -> f32 {
    buffer.layout_runs().map(|lr| lr.line_w).max_by(f32::total_cmp).unwrap()
}