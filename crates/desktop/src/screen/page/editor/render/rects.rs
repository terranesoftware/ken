use crate::screen::{Resources, page::{Palette, editor::{Editor, Layout}}};

pub(crate) fn prepare(
    editor: &Editor,
    layout: Layout,
    palette: Palette,
    resources: &Resources
) {
    let mut all = Vec::new();

    if !layout.focused {
        all.extend(editor.filesystem.read().unwrap().rects(layout, &palette));
        all.push(editor.divider.rect(layout, &palette));
    }
    if let Some(file) = editor.file.read().unwrap().as_ref() {
        all.extend(file.rects(layout, &palette));
    }

    resources.rects(&all);
}