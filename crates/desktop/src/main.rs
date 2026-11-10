mod app;
use app::App;

use tokio::main;
use winit::{error::EventLoopError, event_loop::EventLoop};

#[main]
async fn main() -> Result<(), EventLoopError> {
    let el = EventLoop::new()?;
    
    el.run_app(&mut App::new().await)
}