use tokio::spawn;
use winit::event_loop::ActiveEventLoop;

use crate::{App, Page, UserEvent, app::{Resources, window::Window}};

impl App {
    pub(super) fn ue(
        &mut self,
        ael: &ActiveEventLoop,
        ue: UserEvent,
    ) {
        match ue {
            UserEvent::Resources(result) => match result {
                Ok((result, text)) => match result {
                    Ok(gpu) => {
                        let resources = Resources::new(gpu, text);

                        // Find all the pending windows and drain them
                        self.windows.extract_if(|_, value| matches!(value, Window::Pending(_))).for_each(|(id, window)| {
                            let gpu = resources.gpu.clone();
                            let elp = self.elp.clone();
                            let Window::Pending(window) = window else {
                                unreachable!();
                            };
                            
                            spawn(async move {
                                let result = Page::landing(gpu, window).await;
                                _ = elp.send_event(UserEvent::Window(id, result));
                            });
                        });
                        
                        self.resources = Some(resources);
                    },
                    Err(err) => {
                        eprintln!("{:?}\nExiting...", err);
                        ael.exit();
                    }
                }
                Err(err) => {
                    eprintln!("{:?}\nExiting...", err);
                    ael.exit();
                }
            }
            UserEvent::Window(id, result) => match result {
                Ok(page) => *self.windows.get_mut(&id).unwrap() = Window::Ready(page),
                Err(err) => {
                    eprintln!("{:?}\nExiting...", err);
                    ael.exit();
                }
            }
        }
    }
}