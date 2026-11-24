use crate::{Error, screen::{Resources, page::{Palette, editor::{Editor, Layout}}}};

pub(crate) fn prepare(
    editor: &Editor,
    layout: Layout,
    palette: Palette,
    resources: &Resources
) -> Result<(), Error> {
    let fs = &mut resources.text.fs.write().unwrap();

    let height = layout.h - 89.0;

    if let Some(file) = editor.file.write().unwrap().as_mut() {
        file.shape(fs, height);
    }
    editor.filesystem.write().unwrap().shape(fs, height);

    let ink = palette.ink.text();
    let mut areas = Vec::new();
    
    let filesystem = editor.filesystem.read().unwrap();
    if !layout.focused {
        areas.extend(filesystem.areas(layout, ink));
    }
    
    let file = editor.file.read().unwrap();
    if let Some(file) = file.as_ref() {
        areas.extend(file.areas(layout, ink));
    }

    resources.areas(areas, fs)
}
