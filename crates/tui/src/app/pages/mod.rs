mod editor;
pub(super) use editor::editor;

mod landing;
pub(super) use landing::landing;

pub(super) enum Page {
    Editor,
    Landing
}