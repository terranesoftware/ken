/* mod cache;
use cache::Cache; */

mod handler;

mod present;

pub(super) mod resources;
use resources::Resources;

mod window;
use window::Window;

use std::{collections::HashMap, sync::Arc};

use tokio::spawn;
use winit::{event_loop::EventLoopProxy, window::WindowId};

use crate::UserEvent;

pub(super) struct App {
    // cache: Cache,
    elp: Arc<EventLoopProxy<UserEvent>>,
    resources: Option<Resources>,
    windows: HashMap<WindowId, Window>
}

impl App {
    pub(super) fn new(elp: EventLoopProxy<UserEvent>) -> Self {
        // Wrap it in an Arc because on some platforms, cloning it is expensive
        let elp = Arc::new(elp);
        
        // Asynchronously initialize the app's resources
        let moved = elp.clone();
        spawn(async move {
            let result = Resources::init().await;
            _ = moved.send_event(UserEvent::Resources(result));
        });

        Self {
            // cache: Cache::new(),
            elp,
            resources: None,
            windows: HashMap::new()
        }
    }
}