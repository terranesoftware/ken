use winit::{application::ApplicationHandler, dpi::PhysicalPosition, event::WindowEvent, event_loop::ActiveEventLoop, window::{Window, WindowId}};

use crate::{App, UserEvent, app::rendering::render};

// Will probably add retries or see what type of error it is before exiting
impl ApplicationHandler<UserEvent> for App {
    fn resumed(
        &mut self,
        ael: &ActiveEventLoop
    ) {
        // Add .with_decorations(false) when I create custom window decorations
        let window = ael.create_window(Window::default_attributes().with_title("ken")).unwrap();

        // Center the window
        if let Some(monitor) = window.current_monitor() {
            let origin = monitor.position();
            let screen = monitor.size();
            let size = window.outer_size();

            window.set_outer_position(PhysicalPosition::new(
                origin.x + (screen.width as i32 - size.width as i32) / 2,
                origin.y + (screen.height as i32 - size.height as i32) / 2
            ));
        }

        // Maximize it
        window.set_maximized(true);

        self.window = Some(window);

        if let Err(err) = self.gpu() {
            eprintln!("{:?}\nExiting...", err);
            ael.exit();
        }

        self.window.as_ref().unwrap().request_redraw();
    }

    fn user_event(
        &mut self,
        ael: &ActiveEventLoop,
        ue: UserEvent
    ) {
        match ue {
            UserEvent::FontSystem(fs) => {
                self.fs = Some(fs);

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            },
            UserEvent::Gpu(result) => match result {
                Ok(gpu) => {
                    self.gpu = Some(gpu);
                    
                    if let Err(err) = self.gpu() {
                        eprintln!("{:?}\nExiting...", err);
                        ael.exit();
                    }
                    
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                },
                Err(err) => {
                    eprintln!("{:?}\nExiting...", err);
                    ael.exit();
                }
            }
            UserEvent::Text(text) => {
                self.gpu.as_mut().unwrap().text = Some(text);

                self.text();
            }
        }
    }

    fn window_event(
        &mut self,
        ael: &ActiveEventLoop,
        window: WindowId,
        we: WindowEvent,
    ) {
        match we {
            // Placeholder
            WindowEvent::CloseRequested => ael.exit(),
            WindowEvent::RedrawRequested => {
                render(self);
            }
            _ => {}
        }
    }
}