use tokio::spawn;
use winit::{event::{ElementState, MouseButton, WindowEvent}, event_loop::ActiveEventLoop, window::WindowId};

use crate::{App, UserEvent};

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

                        _ = melp.send_event(UserEvent::Render(wid, result));
                    });
                }
            }
            WindowEvent::MouseInput { button, state, .. } => {
                if matches!(
                    (button, state),
                    (MouseButton::Left, ElementState::Released)
                ) {
                    if let Some(screen) = self.screens.get(&wid) {
                        let surface = screen.page.clicked(self.position);
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