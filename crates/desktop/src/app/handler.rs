use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, window::{Window, WindowId}};

use crate::{App, Error, Gpu};

// Will probably add retries or see what type of error it is before exiting
impl ApplicationHandler<Result<Gpu, Error>> for App {
    fn resumed(
        &mut self,
        ael: &ActiveEventLoop
    ) {
        // Add .with_decorations(false) when I create custom window decorations
        self.window = Some(ael.create_window(Window::default_attributes().with_title("ken")).unwrap());

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