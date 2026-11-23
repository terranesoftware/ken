use glyphon::FontSystem;

use crate::screen::page::landing::Landing;

pub(super) fn shape(
    fs: &mut FontSystem,
    landing: &Landing
) {
    landing.name.write().unwrap().0.shape_until_scroll(fs, false);
    landing.new.write().unwrap().shape(fs);
    landing.open.write().unwrap().shape(fs);
    landing.re.write().unwrap().0.shape_until_scroll(fs, false);
    landing.recent.write().unwrap().0.shape_until_scroll(fs, false);
    landing.tagline.write().unwrap().0.shape_until_scroll(fs, false);
}