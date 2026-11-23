mod components;
use components::Action;

mod instances;
use instances::{name, new, open, re, recent, separator, tagline};

mod layout;
pub(crate) use layout::Layout;

mod methods;

mod render;

use std::sync::{Arc, RwLock};

use carreau::Rect;
use glyphon::{Buffer, Color, Family, TextArea, Weight};
use winit::dpi::PhysicalSize;

use crate::{Error, screen::{Page, Resources, page::{PageMethods, Palette}}};

// Random list of fonts to load, will grow based on what I feel
const FAMILIES: [(Family, Weight); 4] = [
    (Family::Name("Bricolage Grotesque"), Weight::BOLD),
    (Family::Name("Fraunces"), Weight::BOLD),
    (Family::Name("Instrument Serif"), Weight::NORMAL),
    (Family::Name("Righteous"), Weight::NORMAL)
];

type Area = fn(&Buffer, Layout, Color) -> TextArea<'_>;

pub(crate) struct Landing {
    layout: RwLock<Layout>,
    name: RwLock<(Buffer, Area)>,
    new: RwLock<Action>,
    open: RwLock<Action>,
    re: RwLock<(Buffer, Area)>, // Recents Empty
    recent: RwLock<(Buffer, Area)>,
    separator: fn(Layout, &Palette) -> Rect,
    tagline: RwLock<(Buffer, Area)>
}

impl Page {
    pub(crate) async fn landing(
        palette: &Palette,
        resources: &Arc<Resources>,
        scale: f32,
        size: PhysicalSize<u32>
    ) -> Result<Self, Error> {
        let landing = Arc::new(Landing {
            layout: RwLock::new(Layout::default()),
            
            name: name(palette),
            tagline: tagline(),
            new: new(palette),
            open: open(palette),
            separator,
            re: re(),
            recent: recent()
        });

        landing.prepare(Layout::new(scale, size), *palette, resources).await?;
        landing.render(palette, resources)?;
        
        Ok(Self::Landing(landing))
    }
}