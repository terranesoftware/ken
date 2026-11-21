use crate::{Error, Screen, screen::Page};

pub(crate) enum EventKind {
    Navigate(Result<Page, Error>),
    Screen(Result<Screen, Error>),
    Update(Result<(), Error>)
}