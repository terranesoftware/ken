mod window;

use std::sync::Arc;

use tokio::spawn;
use winit::event_loop::ActiveEventLoop;

use crate::{App, Error, Screen, UserEvent};

impl App {
    pub(super) fn r(
        &mut self,
        ael: &ActiveEventLoop
    ) -> Result<(), Error> {
        // Create the window
        let window = self.window(ael);

        let elp = self.elp.clone();
        let gpu = self.resources.gpu.clone();
        let text = self.resources.text.clone();
        let window = Arc::new(window);
        
        // Have to create the surface on the main thread
        let instance = &gpu.instance;
        let surface = instance.create_surface(window.clone())?;
        
        spawn(async move {
            let id = window.id();
            let result = Screen::new(gpu, surface, text, window).await;
            
            _ = elp.send_event(UserEvent::Screen(id, result));
        });

        Ok(())
    }
}