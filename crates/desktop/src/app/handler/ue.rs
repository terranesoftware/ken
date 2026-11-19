use winit::event_loop::ActiveEventLoop;

use crate::{App, Error, UserEvent};

impl App {
    pub(super) fn ue(
        &mut self,
        ael: &ActiveEventLoop,
        ue: UserEvent,
    ) -> Result<(), Error> {
        match ue {
            UserEvent::Render(id, result) => match result {
                Ok(()) => {
                    let screen = self.screens.get(&id).unwrap();

                    screen.window.request_redraw();
                },
                Err(err) => {
                    eprintln!("{:?}\nExiting...", err);
                    ael.exit();
                }
            }
            UserEvent::Screen(id, result) => match result {
                Ok(screen) => {
                    let window = screen.window.clone();
                    _ = self.screens.insert(id, screen);

                    window.request_redraw();
                },
                Err(err) => {
                    eprintln!("{:?}\nExiting...", err);
                    ael.exit();
                }
            }
        }

        Ok(())
    }
}