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
            WindowEvent::Occluded(false) => if let Some(window) = self.windows.get(&window) && let Window::Ready(page) = window {
                page.window().request_redraw();
            }
            _ => {}
        }
    }
}