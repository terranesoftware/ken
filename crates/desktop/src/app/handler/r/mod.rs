mod window;

use std::sync::Arc;

use tokio::spawn;
use wgpu::{SurfaceTarget, rwh::{HasDisplayHandle, HasWindowHandle}};
use winit::event_loop::ActiveEventLoop;

use crate::{App, Error, Page, UserEvent, app::Window};

impl App {
    pub(super) fn r(
        &mut self,
        ael: &ActiveEventLoop
    ) -> Result<(), Error> {
        // Create the window
        let window = self.window(ael);

        // If the GPU is available, wrap the window right now
        if let Some(resources) = &self.resources {
            let elp = self.elp.clone();
            let gpu = resources.gpu.clone();
            let text = resources.text.clone();
            let window = Arc::new(window);

            // Have to create the surface on the main thread
            let instance = &gpu.instance;
            let surface = instance.create_surface(window.clone())?;
            
            spawn(async move {
                let id = window.id();
                let result = Page::landing(gpu, surface, text, window).await;
                
                _ = elp.send_event(UserEvent::Window(id, result));
            });
        }
        // Otherwise, put it in the map as pending
        else {
            self.windows.insert(window.id(), Window::Pending(window));
        }

        Ok(())
    }
}