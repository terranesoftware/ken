use super::{Palette, Rgb};

pub(crate) const DARK: Palette = Palette {
    accent: Rgb(231, 91, 80),
    border: Rgb(52, 59, 66),
    comment: Rgb(133, 138, 144),
    cl: Rgb(35, 28, 31),
    faint: Rgb(119, 126, 133),
    gutter: Rgb(113, 121, 130),
    hover: Rgb(36, 43, 48),
    ink: Rgb(238, 232, 220),
    keyword: Rgb(223, 156, 132),
    muted: Rgb(148, 151, 155),
    paper: Rgb(21, 24, 28),
    selected: Rgb(38, 43, 49),
    shell: Rgb(25, 30, 34),
    sb: Rgb(41, 47, 53),
    r#type: Rgb(165, 185, 159)
};

pub(crate) const LIGHT: Palette = Palette {
    accent: Rgb(183, 60, 50),
    border: Rgb(215, 205, 188),
    comment: Rgb(167, 155, 138),
    cl: Rgb(244, 232, 221),
    faint: Rgb(167, 159, 147),
    gutter: Rgb(181, 170, 153),
    hover: Rgb(240, 230, 216),
    ink: Rgb(41, 40, 35),
    keyword: Rgb(165, 65, 55),
    muted: Rgb(135, 127, 114),
    paper: Rgb(248, 244, 233),
    selected: Rgb(231, 220, 204),
    shell: Rgb(241, 234, 223),
    sb: Rgb(233, 225, 213),
    r#type: Rgb(68, 103, 93)
};