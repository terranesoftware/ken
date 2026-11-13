use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::WindowId};

use crate::{App, app::window::Window};

impl App {
    pub(super) fn we(
        &mut self,
        ael: &ActiveEventLoop,
        we: WindowEvent,
        window: WindowId
    ) {
        match we {
            // Placeholder
            WindowEvent::CloseRequested => ael.exit(),
            WindowEvent::RedrawRequested => self.present(window),
            WindowEvent::Occluded(false) => match self.windows.get(&window).unwrap() {
                Window::Ready(page) => page.window().request_redraw(),
                _ => ()
            }
            _ => {}
        }
    }
}