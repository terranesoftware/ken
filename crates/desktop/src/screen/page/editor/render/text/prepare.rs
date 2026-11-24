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
    if let Some(filesystem) = editor.filesystem.write().unwrap().as_mut() {
        filesystem.shape(fs, height);
    }

    let mut areas = Vec::new();
    let ink = palette.ink.text();
    
    let filesystem = editor.filesystem.read().unwrap();
    let x = filesystem.as_ref().map_or(0.0, |filesystem| {
        areas.extend(filesystem.areas(layout, ink, 0.0));
        
        layout.de()
    });
    
    let file = editor.file.read().unwrap();
    if let Some(file) = file.as_ref() {
        areas.extend(file.areas(layout, ink, x));
    }

    resources.areas(areas, fs)
}
