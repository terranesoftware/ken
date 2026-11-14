use glyphon::{Buffer, Color, TextArea, TextBounds};

// Creates a TextArea with defaults
pub(super) fn area<'a>(
    buffer: &'a Buffer,
    left: f32,
    top: f32
) -> TextArea<'a> {
    TextArea {
        bounds: TextBounds::default(),
        buffer,
        custom_glyphs: &[],
        default_color: Color::rgb(41, 40, 35),
        left,
        scale: 1.0,
        top
    }
}

pub(super) fn center(
    buffer: &Buffer,
    ww: f32 // Window Width
) -> f32 {
    (ww - width(buffer)) / 2.0
}

// Gets the width of the buffer by finding the line with the maximum width
pub(super) fn width(buffer: &Buffer) -> f32 {
    buffer.layout_runs().map(|lr| lr.line_w).max_by(f32::total_cmp).unwrap()
}