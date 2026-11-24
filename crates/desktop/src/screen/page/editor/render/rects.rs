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
    all.extend(editor.file.read().unwrap().rects(layout, &palette));

    resources.rects(&all);
}