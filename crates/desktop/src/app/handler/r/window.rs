#[cfg(target_os = "macos")]
use winit::platform::macos::WindowAttributesExtMacOS;

use winit::{dpi::PhysicalPosition, event_loop::ActiveEventLoop, window::Window};

use crate::App;

impl App {
    pub(super) fn window(
        &mut self,
        ael: &ActiveEventLoop
    ) -> Window {
        let attributes = Window::default_attributes()
            .with_title("ken");
        
        #[cfg(target_os = "macos")]
        let attributes = attributes
            .with_fullsize_content_view(true)
            .with_title_hidden(true)
            .with_titlebar_transparent(true);
        
        let window = ael.create_window(attributes).unwrap();
        
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

        window
    }
}