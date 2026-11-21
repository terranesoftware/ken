/* mod cache;
use cache::Cache; */

mod handler;

mod present;

use std::{collections::HashMap, sync::Arc};

use winit::{dpi::PhysicalPosition, event_loop::EventLoopProxy, keyboard::ModifiersState, window::WindowId};

use crate::{Resources, Screen, UserEvent};

pub(super) struct App {
    // cache: Cache,
    elp: Arc<EventLoopProxy<UserEvent>>,
    ms: ModifiersState,
    position: PhysicalPosition<f64>,
    resources: Resources,
    screens: HashMap<WindowId, Screen>
}

impl App {
    pub(super) fn new(
        elp: EventLoopProxy<UserEvent>,
        resources: Resources
    ) -> Self {
        Self {
            // cache: Cache::new(),
            // Wrap it in an Arc because on some platforms, cloning it is expensive
            elp: Arc::new(elp),
            ms: ModifiersState::default(),
            position: PhysicalPosition::default(),
            resources,
            screens: HashMap::new()
        }
    }
}