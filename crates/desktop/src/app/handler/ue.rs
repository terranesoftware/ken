use std::sync::Arc;

use winit::event_loop::ActiveEventLoop;

use crate::{App, Error, UserEvent, event::EventKind};

impl App {
    pub(super) fn ue(
        &mut self,
        ael: &ActiveEventLoop,
        ue: UserEvent,
    ) -> Result<(), Error> {
        let wid = ue.wid;
        match ue.ek {
            EventKind::Navigate(result) => match result {
                Ok(page) => {
                    let screen = self.screens.get_mut(&wid).unwrap();

                    // Unlock it and replace it with the new page
                    screen.locked = false;
                    screen.page = Arc::new(page);
                    
                    // Request redraw
                    screen.window.request_redraw();
                }
                Err(err) => {
                    eprintln!("{:?}\nExiting...", err);
                    ael.exit();
                }
            }
            EventKind::Screen(result) => match result {
                Ok(screen) => {
                    let window = screen.window.clone();
                    _ = self.screens.insert(wid, screen);

                    window.request_redraw();
                },
                Err(err) => {
                    eprintln!("{:?}\nExiting...", err);
                    ael.exit();
                }
            }
            EventKind::Update(result) => match result {
                Ok(()) => {
                    let screen = self.screens.get(&wid).unwrap();

                    if let Err(err) = screen.resources.render(&screen.palette) {
                        eprintln!("{:?}\nExiting...", err);
                        ael.exit();
                    }
                    screen.window.request_redraw();
                }
                Err(err) => {
                    eprintln!("{:?}\nExiting...", err);
                    ael.exit();
                }
            }
        }

        Ok(())
    }
}