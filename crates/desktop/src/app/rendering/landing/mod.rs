mod text;

use glyphon::{FontSystem, SwashCache};

use winit::window::Window;

use crate::{Error, Gpu, app::buffers::Landing};

pub(crate) fn landing(
    fs: &mut FontSystem,
    gpu: &mut Gpu,
    landing: &mut Landing,
    swc: &mut SwashCache,
    window: &Window
) -> Result<(), Error> {

    // rects()
    // text::render(&mut rp, ta, tr, viewport);

    Ok(())
}