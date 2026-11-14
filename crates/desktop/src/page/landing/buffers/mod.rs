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

use glyphon::{Buffer, Family, FontSystem, Weight};

// Random list of fonts to load, will grow based on what I feel
const FAMILIES: [(Family, Weight); 4] = [
    (Family::Name("Bricolage Grotesque"), Weight::BOLD),
    (Family::Name("Fraunces"), Weight::BOLD),
    (Family::Name("Instrument Serif"), Weight::NORMAL),
    (Family::Name("Righteous"), Weight::NORMAL)
];

pub(crate) struct Buffers {
    pub(super) name: Buffer,
    pub(super) nc: Buffer, // New Command
    pub(super) new: Buffer,
    pub(super) oc: Buffer, // Open Command
    pub(super) open: Buffer,
    pub(super) re: Buffer, // Recents Empty
    pub(super) recent: Buffer,
    pub(super) tagline: Buffer
}

impl Buffers {
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

    pub(super) fn shape(
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