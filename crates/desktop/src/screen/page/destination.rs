use std::path::PathBuf;

pub(crate) enum Destination {
    Editor(Option<PathBuf>),
    Landing
}