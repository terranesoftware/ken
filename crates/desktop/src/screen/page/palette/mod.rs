mod rgb;
pub(super) use rgb::Rgb;

pub(crate) mod themes;

pub(crate) struct Palette {
    pub(crate) accent: Rgb,
    pub(crate) border: Rgb,
    pub(crate) comment: Rgb,
    pub(crate) cl: Rgb, // Cursor line
    pub(crate) faint: Rgb,
    pub(crate) gutter: Rgb,
    pub(crate) hover: Rgb,
    pub(crate) ink: Rgb,
    pub(crate) keyword: Rgb,
    pub(crate) muted: Rgb,
    pub(crate) paper: Rgb,
    pub(crate) selected: Rgb,
    pub(crate) shell: Rgb,
    pub(crate) sb: Rgb, // Subtle border
    pub(crate) r#type: Rgb
}