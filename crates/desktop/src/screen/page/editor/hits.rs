use carreau::Rect;

#[derive(Default)]
pub(crate) struct Hits {
    pub(crate) file: Option<Rect>,
    pub(crate) filesystem: Option<Rect>
}
