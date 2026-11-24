use crate::screen::{Resources, page::{Palette, editor::{Editor, Layout}}};

pub(crate) fn prepare(
    editor: &Editor,
    layout: Layout,
    palette: Palette,
    resources: &Resources
) {
    let mut all = Vec::new();
    
    // Add the filesystem pane and divider rects *and then* return the divider edge so that the file pane knows where to start
    // Otherwise, just return 0.0 (so the file pane starts all the way from the right)
    let x = editor.filesystem.read().unwrap().as_ref().map_or(0.0, |filesystem| {
        all.extend(filesystem.rects(layout, &palette, 0.0));
        all.push(editor.divider.rect(layout, &palette));
        
        layout.de()
    });
    
    if let Some(file) = editor.file.read().unwrap().as_ref() {
        all.extend(file.rects(layout, &palette, x));
    }

    resources.rects(&all);
}