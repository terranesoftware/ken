use std::sync::Arc;

use tokio::spawn;
use winit::{event_loop::EventLoopProxy, window::WindowId};

use crate::{Screen, UserEvent, event::EventKind, screen::{Page, editor, landing}};

pub(super) fn update(
    elp: Arc<EventLoopProxy<UserEvent>>,
    screen: &mut Screen,
    wid: WindowId
) {
    let palette = screen.palette;
    let s = (screen.zoom * screen.window.scale_factor()) as f32;
    let size = screen.window.inner_size();

    let mr = screen.resources.clone();
    match screen.page.as_ref() {
        Page::Editor(editor) => {
            let editor = editor.clone();
            
            _ = spawn(async move {
                let fw = *editor.fw.read().unwrap();
                let result = editor.prepare(editor::Layout::new(fw, s, size), palette, &mr).await;
                
                _ = elp.send_event(UserEvent {
                    ek: EventKind::Update(result),
                    wid
                });
            });
        },
        Page::Landing(landing) => {
            let landing = landing.clone();
            
            _ = spawn(async move {
                let result = landing.prepare(landing::Layout::new(s, size), palette, &mr).await;
                
                _ = elp.send_event(UserEvent {
                    ek: EventKind::Update(result),
                    wid
                });
            });
        }
    }
}