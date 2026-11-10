mod app;
use app::App;

mod error;
use error::Error;

use tokio::main;
use winit::event_loop::EventLoop;

use crate::app::Gpu;

#[main]
async fn main() -> Result<(), Error> {
    // Create a user callback for GPU initialization
    let el = EventLoop::<Result<Gpu, Error>>::with_user_event().build()?;

    // Pass it in during app creation
    let mut app = App::new(el.create_proxy());
    
    el.run_app(&mut app)?;

    Ok(())
}