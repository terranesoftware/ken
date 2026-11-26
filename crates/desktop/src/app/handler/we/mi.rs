use tokio::spawn;
use winit::{event::{ElementState, MouseButton}, window::WindowId};

use crate::{App, UserEvent, app::handler::we::update, event::EventKind, screen::{Destination, Page, PageMethods}};

pub(super) fn mi(
    app: &mut App,
    button: MouseButton,
    state: ElementState,
    wid: WindowId
) {
    // Update whether the mouse is held down or not
    match state {
        ElementState::Pressed => app.mh = true,
        ElementState::Released => app.mh = false
    }
    
    if matches!(button, MouseButton::Left) && !app.mh {
        if let Some(screen) = app.screens.get_mut(&wid) && !screen.locked {
            if let Some(destination) = screen.page.clicked(app.position, &screen.window) {
                screen.locked = true;
                
                match destination {
                    Destination::Editor(path) => {
                        let melp = app.elp.clone();
                        let mk = app.ken.clone();
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
            }
            else if matches!(*screen.page, Page::Editor(_)) {
                update(app.elp.clone(), screen, wid);
            }
        }
    }
}