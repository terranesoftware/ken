use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::WindowId};

use crate::App;

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
            WindowEvent::Occluded(false) => {
                
            },
            _ => {}
        }
    }
}