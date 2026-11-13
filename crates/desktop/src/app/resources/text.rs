use glyphon::{FontSystem, SwashCache};

pub(crate) struct Text {
    pub(super) fs: FontSystem,
    pub(super) sc: SwashCache
}

impl Text {
    pub(super) fn new() -> Self {
        let mut fs = FontSystem::new();
        
        let database = fs.db_mut();
        database.load_font_data(include_bytes!("../../../fonts/BricolageGrotesque-Bold.ttf").to_vec());
        database.load_font_data(include_bytes!("../../../fonts/Fraunces_72pt-Bold.ttf").to_vec());
        database.load_font_data(include_bytes!("../../../fonts/InstrumentSerif-Regular.ttf").to_vec());
        database.load_font_data(include_bytes!("../../../fonts/Righteous-Regular.ttf").to_vec());

        Self {
            fs,
            sc: SwashCache::new()
        }
    }
}