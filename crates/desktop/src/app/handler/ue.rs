use std::sync::Arc;

use tokio::spawn;
use winit::event_loop::ActiveEventLoop;

use crate::{App, Error, Page, UserEvent, app::{Resources, window::Window}};

impl App {
    pub(super) fn ue(
        &mut self,
        ael: &ActiveEventLoop,
        ue: UserEvent,
    ) -> Result<(), Error> {
        match ue {
            UserEvent::Resources(result) => match result {
                Ok((result, text)) => match result {
                    Ok(gpu) => {
                        let resources = Resources::new(gpu, text);

                        // Find all the pending windows and drain them
                        self.windows.extract_if(|_, value| matches!(value, Window::Pending(_))).try_for_each(|(id, window)| {
                            let elp = self.elp.clone();
                            let gpu = resources.gpu.clone();
                            let text = resources.text.clone();
                            let window = match window {
                                Window::Pending(window) => Arc::new(window),
                                _ => unreachable!()
                            };

                            let instance = &gpu.instance;
                            let surface = instance.create_surface(window.clone())?;
                            
                            spawn(async move {
                                let result = Page::landing(gpu, surface, text, window).await;
                                
                                _ = elp.send_event(UserEvent::Window(id, result));
                            });

                            Ok::<(), Error>(())
                        })?;
                        
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
                Ok(page) => {
                    let window = page.window();
                    _ = self.windows.insert(id, Window::Ready(page));

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