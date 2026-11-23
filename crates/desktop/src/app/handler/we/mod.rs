mod update;
use update::update;

use tokio::spawn;
use winit::{event::{ElementState, MouseButton, WindowEvent}, event_loop::ActiveEventLoop, keyboard::Key, window::{Theme, WindowId}};

use crate::{App, UserEvent, event::EventKind, screen::{Destination, Page, PageMethods, themes::{DARK, LIGHT}}};

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
                    screen.window.set_cursor(screen.page.moved(&screen.palette, position, &screen.resources));

                    if let Err(err) = screen.page.render(&screen.palette, &screen.resources) {
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
                                update(elp, screen, wid);
                            }
                        }
                        // Zoom out
                        Key::Character("-") => if let Some(screen) = self.screens.get_mut(&wid) {
                            if screen.zoom > 0.3 {
                                screen.zoom -= 0.1;
                                
                                let elp = self.elp.clone();
                                update(elp, screen, wid);
                            }
                        }
                        _ => ()
                    }
                }
            }
            WindowEvent::ModifiersChanged(modifiers) => self.ms = modifiers.state(),
            WindowEvent::MouseInput { button, state, .. } => {
                // Update whether the mouse is held down or not
                match state {
                    ElementState::Pressed => self.mh = true,
                    ElementState::Released => self.mh = false
                }
                
                if matches!(button, MouseButton::Left) && !self.mh {
                    if let Some(screen) = self.screens.get_mut(&wid) && !screen.locked {
                        if let Some(destination) = screen.page.clicked(self.position, &screen.window) {
                            screen.locked = true;
                            
                            match destination {
                                Destination::Editor(path) => {
                                    let melp = self.elp.clone();
                                    let mk = self.ken.clone();
                                    let mr = screen.resources.clone();
                                    let palette = screen.palette;
                                    let scale = (screen.window.scale_factor() * screen.zoom) as f32;
                                    let size = screen.window.inner_size();
                                    spawn(async move {
                                        let result = Page::editor(mk, &palette, path, &mr, scale, size).await;

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
            WindowEvent::Occluded(false) => {
                if let Some(screen) = self.screens.get(&wid) {
                    screen.window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => self.present(wid),
            WindowEvent::ThemeChanged(theme) => {
                if let Some(screen) = self.screens.get_mut(&wid) {
                    screen.palette = match theme {
                        Theme::Dark => DARK,
                        Theme::Light => LIGHT
                    };

                    update(self.elp.clone(), screen, wid);
                }
            }
            _ => {}
        }
    }
}