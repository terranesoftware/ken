mod cm;
use cm::cm;

mod ki;
use ki::ki;

mod mi;
use mi::mi;

mod update;
use update::update;

use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::{Theme, WindowId}};

use crate::{App, screen::themes::{DARK, LIGHT}};

impl App {
    pub(super) fn we(
        &mut self,
        ael: &ActiveEventLoop,
        we: WindowEvent,
        wid: WindowId
    ) {
        match we {
            // Placeholder
            WindowEvent::CloseRequested => ael.exit(),
            
            WindowEvent::CursorMoved { position, .. } => cm(ael, self, position, wid),
            
            WindowEvent::KeyboardInput { event, .. } => ki(self, event, wid),
            
            WindowEvent::ModifiersChanged(modifiers) => self.ms = modifiers.state(),
            
            WindowEvent::MouseInput { button, state, .. } => mi(self, button, state, wid),
            
            WindowEvent::Occluded(false) => if let Some(screen) = self.screens.get(&wid) {
                screen.window.request_redraw();
            }
            
            WindowEvent::RedrawRequested => self.present(wid),
            
            WindowEvent::ThemeChanged(theme) => if let Some(screen) = self.screens.get_mut(&wid) {
                screen.palette = match theme {
                    Theme::Dark => DARK,
                    Theme::Light => LIGHT
                };
                
                update(self.elp.clone(), screen, wid);
            }
            
            _ => {}
        }
    }
}