mod app;
use app::App;

mod error;
use error::Error;

mod event;
use event::UserEvent;

mod resources;
use resources::Resources;

mod screen;
use screen::Screen;

use tokio::main;
use winit::event_loop::EventLoop;

#[main]
async fn main() -> Result<(), Error> {
    // Initialize the resources
    let resources = Resources::new().await?;
    
    // Create a user callback for GPU initialization
    let el = EventLoop::<UserEvent>::with_user_event().build()?;

    // Pass it in during app creation
    let mut app = App::new(
        el.create_proxy(),
        resources
    );
    
    el.run_app(&mut app)?;

    Ok(())
}