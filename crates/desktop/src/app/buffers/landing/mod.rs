mod name;
use name::name;

mod nc;
use nc::nc;

mod new;
use new::new;

mod oc;
use oc::oc;

mod open;
use open::open;

mod re;
use re::re;

mod recent;
use recent::recent;

mod tagline;
use tagline::tagline;

use glyphon::{Buffer, FontSystem};

pub(crate) struct Landing {
    pub(crate) name: Buffer,
    pub(crate) nc: Buffer, // New Command
    pub(crate) new: Buffer,
    pub(crate) oc: Buffer, // Open Command
    pub(crate) open: Buffer,
    pub(crate) re: Buffer, // Recents Empty
    pub(crate) recent: Buffer,
    pub(crate) tagline: Buffer
}

impl Landing {
    pub(crate) fn new() -> Self {
        Self {
            name: name(),
            nc: nc(),
            new: new(),
            oc: oc(),
            open: open(),
            re: re(),
            recent: recent(),
            tagline: tagline()
        }
    }

    pub(crate) fn shape(
        &mut self,
        fs: &mut FontSystem
    ) {
        self.name.shape_until_scroll(fs, false);
        self.nc.shape_until_scroll(fs, false);
        self.new.shape_until_scroll(fs, false);
        self.oc.shape_until_scroll(fs, false);
        self.open.shape_until_scroll(fs, false);
        self.re.shape_until_scroll(fs, false);
        self.recent.shape_until_scroll(fs, false);
        self.tagline.shape_until_scroll(fs, false);
    }
}