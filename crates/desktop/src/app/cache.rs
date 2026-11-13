use crate::page::{Editor, Landing};

pub(super) struct Cache {
    editor: Option<Editor>,
    landing: Option<Landing>
}

impl Cache {
    pub(super) fn new() -> Self {
        Self {
            editor: None,
            landing: None
        }
    }
}