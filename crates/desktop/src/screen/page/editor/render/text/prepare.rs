use crate::{Error, screen::{Resources, page::{Palette, editor::{Editor, Layout}}}};

pub(crate) fn prepare(
    editor: &Editor,
    layout: Layout,
    palette: Palette,
    resources: &Resources
) -> Result<(), Error> {
    let fs = &mut resources.text.fs.write().unwrap();

    let height = layout.h - 89.0;

    editor.file.write().unwrap().shape(fs, height);
    editor.filesystem.write().unwrap().shape(fs, height);

    let ink = palette.ink.text();

    let file = editor.file.read().unwrap();
    let filesystem = editor.filesystem.read().unwrap();

    let mut areas = Vec::new();

    if !layout.focused {
        areas.extend(filesystem.areas(layout, ink));
    }
    areas.extend(file.areas(layout, ink));

    resources.areas(areas, fs)
}
