mod zoom;
use zoom::zoom;

use tokio::spawn;
use winit::{event::{ElementState, MouseButton, WindowEvent}, event_loop::ActiveEventLoop, keyboard::Key, window::WindowId};

use crate::{App, UserEvent, event::EventKind, screen::{Destination, Page, PageMethods}};

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
            WindowEvent::CursorMoved { position, .. } => {
                self.position = position;
                
                if let Some(screen) = self.screens.get(&wid) && !screen.locked {
                    screen.page.moved(position, &screen.resources);

                    if let Err(err) = screen.page.render(&screen.resources) {
                        eprintln!("{:?}\nExiting...", err);
                        ael.exit();
                    }
                    
                    screen.window.request_redraw();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                // Zoom
                if event.state.is_pressed() && self.ms.control_key() {
                    match event.logical_key.as_ref() {
                        // Zoom in
                        Key::Character("=") => if let Some(screen) = self.screens.get_mut(&wid) {
                            if screen.zoom < 4.0 {
                                screen.zoom += 0.1;
                                
                                let elp = self.elp.clone();
                                zoom(elp, screen, wid);
                            }
                        }
                        // Zoom out
                        Key::Character("-") => if let Some(screen) = self.screens.get_mut(&wid) {
                            if screen.zoom > 0.3 {
                                screen.zoom -= 0.1;
                                
                                let elp = self.elp.clone();
                                zoom(elp, screen, wid);
                            }
                        }
                        _ => ()
                    }
                }
            }
            WindowEvent::ModifiersChanged(modifiers) => self.ms = modifiers.state(),
            WindowEvent::MouseInput { button, state, .. } => {
                if matches!(
                    (button, state),
                    (MouseButton::Left, ElementState::Released)
                ) {
                    if let Some(screen) = self.screens.get_mut(&wid) && !screen.locked {
                        if let Some(destination) = screen.page.clicked(self.position) {
                            screen.locked = true;
                            
                            match destination {
                                Destination::Editor => {
                                    let melp = self.elp.clone();
                                    let mr = screen.resources.clone();
                                    let scale = screen.window.scale_factor() as f32;
                                    let size = screen.window.inner_size();
                                    spawn(async move {
                                        let result = Page::editor(&mr, scale, size).await;

                                        _ = melp.send_event(UserEvent {
                                            ek: EventKind::Navigate(result),
                                            wid
                                        });
                                    });
                                },
                                Destination::Landing => ()
                            }
                        };
                    }
                }
            }
            WindowEvent::RedrawRequested => self.present(wid),
            WindowEvent::Occluded(false) => {
                if let Some(screen) = self.screens.get(&wid) {
                    screen.window.request_redraw();
                }
            }
            _ => {}
        }
    }
}