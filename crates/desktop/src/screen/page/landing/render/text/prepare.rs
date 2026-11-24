use crate::{Error, screen::{Resources, page::{Palette, landing::{Landing, Layout}}}};

pub(crate) fn prepare(
    landing: &Landing,
    layout: Layout,
    palette: Palette,
    resources: &Resources
) -> Result<(), Error> {
    let fs = &mut resources.text.fs.write().unwrap();

    landing.name.write().unwrap().0.shape_until_scroll(fs, false);
    landing.new.write().unwrap().shape(fs);
    landing.open.write().unwrap().shape(fs);
    landing.re.write().unwrap().0.shape_until_scroll(fs, false);
    landing.recent.write().unwrap().0.shape_until_scroll(fs, false);
    landing.tagline.write().unwrap().0.shape_until_scroll(fs, false);

    let ink = palette.ink.text();

    let name = landing.name.read().unwrap();
    let new = landing.new.read().unwrap();
    let open = landing.open.read().unwrap();
    let re = landing.re.read().unwrap();
    let recent = landing.recent.read().unwrap();
    let tagline = landing.tagline.read().unwrap();

    resources.areas([
        (name.1)(&name.0, layout, ink),
        (tagline.1)(&tagline.0, layout, ink)
    ].into_iter().chain(new.areas(layout, ink)).chain(open.areas(layout, ink)).chain([
        (recent.1)(&recent.0, layout, ink),
        (re.1)(&re.0, layout, ink)
    ]), fs)
}
