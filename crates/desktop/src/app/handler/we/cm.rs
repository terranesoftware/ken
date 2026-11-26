use winit::{dpi::PhysicalPosition, event_loop::ActiveEventLoop, window::WindowId};

use crate::{App, screen::PageMethods};

pub(super) fn cm(
    ael: &ActiveEventLoop,
    app: &mut App,
    position: PhysicalPosition<f64>,
    wid: WindowId
) {
    app.position = position;
    
    if let Some(screen) = app.screens.get(&wid) && !screen.locked {
        screen.window.set_cursor(screen.page.moved(&screen.palette, position, &screen.resources));
        
        if let Err(err) = screen.resources.render(&screen.palette) {
            eprintln!("{:?}\nExiting...", err);
            ael.exit();
        }
        
        screen.window.request_redraw();
    }
}