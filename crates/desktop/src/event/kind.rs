use crate::{Error, Screen, screen::Page};

pub(crate) enum EventKind {
    Navigate(Result<Page, Error>),
    Render(Result<(), Error>),
    Screen(Result<Screen, Error>)
}