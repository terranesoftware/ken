mod kind;
pub(super) use kind::EventKind;

use winit::window::WindowId;

pub(super) struct UserEvent {
    pub(super) ek: EventKind,
    pub(super) wid: WindowId
}
