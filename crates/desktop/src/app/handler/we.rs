use tokio::spawn;
use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::WindowId};

use crate::{App, UserEvent, app::window::Window};

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
                if let Some(window) = self.windows.get(&wid) && let Window::Ready(page) = window {
                    page.moved(position);

                    let melp = self.elp.clone();
                    let mp = page.clone();
                    spawn(async move {
                        let result = mp.render().await;

                        _ = melp.send_event(UserEvent::Render(wid, result));
                    });
                }
            }
            WindowEvent::RedrawRequested => self.present(wid),
            WindowEvent::Occluded(false) => {
                if let Some(window) = self.windows.get(&wid) && let Window::Ready(page) = window {
                    page.window().request_redraw();
                }
            }
            _ => {}
        }
    }
}