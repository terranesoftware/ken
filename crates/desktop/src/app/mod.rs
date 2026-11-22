/* mod cache;
use cache::Cache; */

mod handler;

mod present;

use std::{collections::HashMap, sync::Arc};

use ken::Ken;
use winit::{dpi::PhysicalPosition, event_loop::EventLoopProxy, keyboard::ModifiersState, window::WindowId};

use crate::{Error, Resources, Screen, UserEvent};

pub(super) struct App {
    // cache: Cache,
    elp: Arc<EventLoopProxy<UserEvent>>,
    ken: Ken,
    ms: ModifiersState,
    position: PhysicalPosition<f64>,
    resources: Resources,
    screens: HashMap<WindowId, Screen>
}

impl App {
    pub(super) async fn new(elp: EventLoopProxy<UserEvent>) -> Result<Self, Error> {
        Ok(Self {
            // cache: Cache::new(),
            // Wrap it in an Arc because on some platforms, cloning it is expensive
            elp: Arc::new(elp),
            ken: Ken::new(),
            ms: ModifiersState::default(),
            position: PhysicalPosition::default(),
            resources: Resources::new().await?,
            screens: HashMap::new()
        })
    }
}