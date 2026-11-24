use carreau::Rect;

use crate::screen::Resources;

impl Resources {
    pub(crate) fn rects(
        &self,
        rects: &[Rect]
    ) {
        self.carreau.write().unwrap().prepare(&self.gpu.queue, rects);
    }
}
