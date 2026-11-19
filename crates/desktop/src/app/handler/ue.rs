use std::sync::Arc;

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
                    let page = self.windows.get(&id).unwrap();

                    page.window().request_redraw();
                },
                Err(err) => {
                    eprintln!("{:?}\nExiting...", err);
                    ael.exit();
                }
            }
            UserEvent::Window(id, result) => match result {
                Ok(page) => {
                    let window = page.window();
                    _ = self.windows.insert(id, Arc::new(page));

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