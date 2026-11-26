use ken::buffers::File;
use winit::{event::KeyEvent, keyboard::{Key, NamedKey}, window::WindowId};

use crate::{App, app::handler::we::update, screen::{Page, editor::Focus}};

pub(super) fn ki(
    app: &mut App,
    event: KeyEvent,
    wid: WindowId
) {
    if event.state.is_pressed() && let Some(screen) = app.screens.get_mut(&wid) && !screen.locked {
        // Zoom
        if app.ms.control_key() {
            match event.logical_key.as_ref() {
                // Zoom in
                Key::Character("=") => if let Some(screen) = app.screens.get_mut(&wid) {
                    if screen.zoom < 4.0 {
                        screen.zoom += 0.1;
                        
                        let elp = app.elp.clone();
                        update(elp, screen, wid);
                    }
                }
                // Zoom out
                Key::Character("-") => if let Some(screen) = app.screens.get_mut(&wid) {
                    if screen.zoom > 0.3 {
                        screen.zoom -= 0.1;
                        
                        let elp = app.elp.clone();
                        update(elp, screen, wid);
                    }
                }
                _ => ()
            }
        }
        else {
            match &*screen.page {
                Page::Editor(editor) => {
                    if let Some(focus) = editor.focus {
                        let mut ken = app.ken.write().unwrap();
                        
                        match event.logical_key {
                            Key::Named(NamedKey::ArrowUp) => match focus {
                                Focus::File => {
                                    let files = &mut ken.files;
                                    
                                    match files.active.as_ref().unwrap() {
                                        File::Ephemeral(key) => {
                                            let (line, _) = &mut files.ephemeral.get_mut(*key).cursor;
                                            
                                            *line = line.saturating_sub(1);
                                        }
                                        File::Persistent(key) => ()
                                    }
                                }
                                Focus::Filesystem => ()
                            },
                            _ => ()
                        }
                    }
                }
                // Probably allow you to scroll through the landing page buttons later
                Page::Landing(landing) => ()
            }
            
        }
    }
}