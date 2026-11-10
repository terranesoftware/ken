use winit::{application::ApplicationHandler, dpi::PhysicalPosition, event::WindowEvent, event_loop::ActiveEventLoop, window::{Window, WindowId}};

use crate::{App, Error, Gpu};

// Will probably add retries or see what type of error it is before exiting
impl ApplicationHandler<Result<Gpu, Error>> for App {
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

        if let Err(err) = self.complete() {
            eprintln!("{:?}\nExiting...", err);
            ael.exit();
        }

        self.window.as_ref().unwrap().request_redraw();
    }

    fn user_event(
        &mut self,
        ael: &ActiveEventLoop,
        event: Result<Gpu, Error>
    ) {
        match event {
            Ok(gpu) => {
                self.gpu = Some(gpu);

                if let Err(err) = self.complete() {
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
                if let Some(gpu) = &self.gpu {
                    gpu.background();
                }
            }
            _ => {}
        }
    }
}