use crate::screen::{Resources, page::{Palette, landing::{Landing, Layout}}};

pub(crate) fn prepare(
    landing: &Landing,
    layout: Layout,
    palette: Palette,
    resources: &Resources
) {
    let rects = [
        landing.new.read().unwrap().rect(layout, &palette),
        landing.open.read().unwrap().rect(layout, &palette),
        (landing.separator)(layout, &palette)
    ];

    resources.carreau.write().unwrap().prepare(&resources.gpu.queue, &rects);
}