mod window;

use tokio::spawn;
use winit::event_loop::ActiveEventLoop;

use crate::{App, Page, UserEvent, app::Window};

impl App {
    pub(super) fn r(
        &mut self,
        ael: &ActiveEventLoop
    ) {
        // Create the window
        let window = self.window(ael);

        // If the GPU is available, wrap the window right now
        if let Some(resources) = &self.resources {
            let me = self.elp.clone();
            let mg = resources.gpu.clone();
            spawn(async move {
                let id = window.id();
                let result = Page::landing(mg, window).await;
                
                _ = me.send_event(UserEvent::Window(id, result));
            });
        }
        // Otherwise, put it in the map as pending
        else {
            self.windows.insert(window.id(), Window::Pending(window));
        }
    }
}