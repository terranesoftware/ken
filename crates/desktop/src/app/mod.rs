/* mod cache;
use cache::Cache; */

mod handler;

mod present;

use std::{collections::HashMap, sync::Arc};

use winit::{event_loop::EventLoopProxy, window::WindowId};

use crate::{Page, Resources, UserEvent};

pub(super) struct App {
    // cache: Cache,
    elp: Arc<EventLoopProxy<UserEvent>>,
    resources: Resources,
    windows: HashMap<WindowId, Arc<Page>>
}

impl App {
    pub(super) fn new(
        elp: EventLoopProxy<UserEvent>,
        resources: Resources
    ) -> Self {
        // Wrap it in an Arc because on some platforms, cloning it is expensive
        let elp = Arc::new(elp);
        
        Self {
            // cache: Cache::new(),
            elp,
            resources,
            windows: HashMap::new()
        }
    }
}