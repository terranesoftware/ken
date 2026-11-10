mod app;
use app::App;

use winit::{error::EventLoopError, event_loop::EventLoop};

fn main() -> Result<(), EventLoopError> {
    let el = EventLoop::new()?;
    
    el.run_app(&mut App::default())
}