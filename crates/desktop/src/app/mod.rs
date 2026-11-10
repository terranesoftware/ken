use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, window::{Window, WindowId}};

#[derive(Default)]
pub(super) struct App {
    window: Option<Window>
}

impl ApplicationHandler for App {
    fn resumed(
        &mut self,
        ael: &ActiveEventLoop
    ) {
        // Add .with_decorations(false) when I create custom window decorations
        self.window = Some(ael.create_window(Window::default_attributes().with_title("ken")).unwrap());
    }

    fn window_event(
        &mut self,
        ael: &ActiveEventLoop,
        window: WindowId,
        we: WindowEvent,
    ) {
        // Placeholder
        if we == WindowEvent::CloseRequested {
            ael.exit();
        }
    }
}