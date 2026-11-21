mod window;

use std::sync::Arc;

use tokio::spawn;
use winit::{event_loop::ActiveEventLoop, window::Theme};

use crate::{App, Error, Screen, UserEvent, event::EventKind};

impl App {
    pub(super) fn r(
        &mut self,
        ael: &ActiveEventLoop,
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
            let theme = window.theme().unwrap_or(Theme::Light);
            let wid = window.id();
            let result = Screen::new(gpu, surface, text, theme, window).await;
            
            _ = elp.send_event(UserEvent {
                ek: EventKind::Screen(result),
                wid
            });
        });

        Ok(())
    }
}