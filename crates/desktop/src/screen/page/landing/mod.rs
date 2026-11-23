mod components;
use components::Action;

mod layout;
pub(crate) use layout::Layout;

mod methods;

use rand::seq::IndexedRandom;

mod render;

use std::sync::{Arc, RwLock};

use carreau::{Rect, rect::Border};
use glyphon::{Attrs, Buffer, Color, Family, Metrics, Shaping, TextArea, Weight};
use winit::dpi::PhysicalSize;

use crate::{Error, screen::{Page, Resources, page::{PageMethods, Palette, landing::render::text::{area, center, width}}}};

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
            
            name: RwLock::new((
                {
                    let mut buffer = Buffer::new_empty(Metrics::new(100.0, f32::MIN_POSITIVE));
                    
                    let (family, weight) = *FAMILIES.choose(&mut rand::rng()).unwrap();
                    let attrs = Attrs::new().family(family).weight(weight);
                    
                    buffer.set_rich_text([
                        ("ken", attrs.clone().color(palette.ink.text())),
                        (".", attrs.clone().color(palette.accent.text()))
                    ], &attrs, Shaping::Advanced, None);
                    
                    buffer
                },
                |buffer, layout, ink| area(buffer, center(buffer, layout.w), layout.top, layout.s, ink)
            )),
            tagline: RwLock::new((
                {
                    let mut buffer = Buffer::new_empty(Metrics::new(37.5, f32::MIN_POSITIVE));
                    
                    let (family, weight) = *FAMILIES.choose(&mut rand::rng()).unwrap();
                    let attrs = Attrs::new().family(family).weight(weight);
                    
                    buffer.set_text("The text editor is back.", &attrs, Shaping::Advanced, None);
                    
                    buffer
                },
                |buffer, layout, ink| area(buffer, center(buffer, layout.w), layout.top + 87.5, layout.s, ink)
            )),
            new: RwLock::new(Action::new(
                ("\u{E0C9}", |buffer, layout, ink| area(buffer, layout.left + 12.5, layout.top + 175.0, layout.s, ink)),
                ("New", |buffer, layout, ink| area(buffer, layout.left + 50.0, layout.top + 175.0, layout.s, ink)),
                palette,
                |layout, backdrop| Rect {
                    backdrop,
                    border: Border::new([0.0; 4], 11.375 * layout.s, 0.0),
                    position: [layout.left * layout.s, (layout.top + 137.5) * layout.s],
                    size: [layout.cw * layout.s, 75.0 * layout.s]
                },
                ("Ctrl+N", |buffer, layout, ink| area(buffer, layout.right - width(buffer) - 12.5, layout.top + 175.0, layout.s, ink))
            )),
            open: RwLock::new(Action::new(
                ("\u{E247}", |buffer, layout, ink| area(buffer, layout.left + 12.5, layout.top + 250.0, layout.s, ink)),
                ("Open", |buffer, layout, ink| area(buffer, layout.left + 50.0, layout.top + 250.0, layout.s, ink)),
                palette,
                |layout, backdrop| Rect {
                    backdrop,
                    border: Border::new([0.0; 4], 11.375 * layout.s, 0.0),
                    position: [layout.left * layout.s, (layout.top + 212.5) * layout.s],
                    size: [layout.cw * layout.s, 75.0 * layout.s]
                },
                ("Ctrl+O", |buffer, layout, ink| area(buffer, layout.right - width(buffer) - 12.5, layout.top + 250.0, layout.s, ink))
            )),
            separator: |layout, palette| Rect {
                backdrop: palette.border.rect(),
                border: Border::new([0.0; 4], 0.0, 0.0),
                position: [layout.left * layout.s, (layout.top + 312.5) * layout.s],
                size: [layout.cw * layout.s, 2.5 * layout.s]
            },
            re: RwLock::new((
                {
                    let mut buffer = Buffer::new_empty(Metrics::new(17.5, 10.0));
                    
                    let attrs = Attrs::new().family(Family::Name("Gelasio")).weight(Weight::NORMAL);
                    
                    buffer.set_text("Nothing here yet.", &attrs, Shaping::Advanced, None);
                    
                    buffer
                },
                |buffer, layout, ink| area(buffer, layout.left, layout.top + 425.0, layout.s, ink)
            )),
            recent: RwLock::new((
                {
                    let mut buffer = Buffer::new_empty(Metrics::new(12.5, 8.0));
                    
                    let attrs = Attrs::new().family(Family::Name("Cascadia Code")).weight(Weight::SEMIBOLD);
                    
                    buffer.set_text("RECENT", &attrs, Shaping::Advanced, None);
                    
                    buffer
                },
                |buffer, layout, ink| area(buffer, layout.left, layout.top + 375.0, layout.s, ink)
            ))
        });

        landing.prepare(Layout::new(scale, size), *palette, resources).await?;
        landing.render(palette, resources)?;
        
        Ok(Self::Landing(landing))
    }
}