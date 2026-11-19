use winit::dpi::PhysicalSize;

pub(super) fn layout(wd: PhysicalSize<u32>) -> (f32, f32, f32, f32, f32)  {
    let h = wd.height as f32;
    let w = wd.width as f32;
    
    let cw = w - 2000.0;
    let left = (w - cw) / 2.0;
    let right = cw + left;
    
    let top = (h - 1000.0) / 2.0;

    (cw, left, right, top, w)
}