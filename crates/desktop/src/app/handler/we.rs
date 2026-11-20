use tokio::spawn;
use winit::{event::{ElementState, MouseButton, WindowEvent}, event_loop::ActiveEventLoop, window::WindowId};

use crate::{App, UserEvent, event::EventKind, screen::{Destination, Page}};

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
                if let Some(screen) = self.screens.get(&wid) {
                    screen.page.moved(position, &screen.resources);
                    self.position = position;

                    let melp = self.elp.clone();
                    let mp = screen.page.clone();
                    let mr = screen.resources.clone();
                    spawn(async move {
                        let result = mp.render(&mr).await;

                        _ = melp.send_event(UserEvent {
                            ek: EventKind::Render(result),
                            wid
                        });
                    });
                }
            }
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
                                    let size = screen.window.inner_size();
                                    spawn(async move {
                                        let result = Page::editor(&mr, size).await;

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