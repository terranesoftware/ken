mod r;
mod ue;
mod we;

use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, window:: WindowId};

use crate::{App, UserEvent};

// Will probably add retries or see what type of error it is before exiting
impl ApplicationHandler<UserEvent> for App {
    fn resumed(
        &mut self,
        ael: &ActiveEventLoop
    ) {
        self.r(ael);
    }

    fn user_event(
        &mut self,
        ael: &ActiveEventLoop,
        ue: UserEvent
    ) {
        self.ue(ael, ue);
    }

    fn window_event(
        &mut self,
        ael: &ActiveEventLoop,
        window: WindowId,
        we: WindowEvent,
    ) {
        self.we(ael, we, window);
    }
}