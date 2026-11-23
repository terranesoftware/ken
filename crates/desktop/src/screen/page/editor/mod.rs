mod buffers;
pub(super) use buffers::Buffers;

mod components;
use components::{Divider, Gutter, Heading};

mod instances;
use instances::divider;

mod layout;
pub(crate) use layout::Layout;

mod methods;

mod render;

use std::{env::home_dir, path::{MAIN_SEPARATOR, PathBuf}, sync::{Arc, RwLock}};

use carreau::{Rect, rect::Border};
use glyphon::{Buffer, Color, TextArea};
use ken::Ken;
use wgpu::CommandEncoderDescriptor;
use winit::{dpi::{PhysicalPosition, PhysicalSize}, window::{CursorIcon, Window}};

use crate::{Error, screen::{Page, Resources, page::{Destination, PageMethods, Palette, background, editor::render::{rects, text::{self, area}}}}};

type Area = fn(&Buffer, Layout, Color) -> TextArea<'_>;

pub(crate) struct Editor {
    buffers: RwLock<Buffers>,
    divider: Divider,
    fileg: RwLock<Gutter>, // File gutter
    fileh: RwLock<Heading>, // File heading
    filesg: RwLock<Gutter>, // Filesystem gutter
    filesh: RwLock<Heading>, // Filesystem heading
    fp: fn(Layout, &Palette) -> Rect, // Filesystem pane
    hits: RwLock<Hits>,
    pub(crate) ken: Arc<RwLock<Ken>>,
    pub(crate) layout: RwLock<Layout>
}

impl Page {
    pub(crate) async fn editor(
        ken: Arc<RwLock<Ken>>,
        palette: &Palette,
        path: Option<PathBuf>,
        resources: &Arc<Resources>,
        scale: f32,
        size: PhysicalSize<u32>
    ) -> Result<Self, Error> {
        let focused = if let Some(path) = path {
            let mut ken = ken.write().unwrap();
            ken.active = path.clone();
            ken.directories.insert(path)?;
            
            false
        }
        else {
            true
        };
        
        let editor = Arc::new(Editor {
            buffers: RwLock::new(Buffers::new(&ken, palette)),
            divider: Divider::new(
                |layout, backdrop| Rect {
                    backdrop,
                    border: Border::new([0.0; 4], 0.0, 0.0),
                    position: [(layout.fw - 2.0) * layout.s, 0.0],
                    size: [6.0 * layout.s, layout.h * layout.s]
                },
                |layout, backdrop| Rect {
                    backdrop,
                    border: Border::new([0.0; 4], 0.0, 0.0),
                    position: [layout.fw * layout.s, 0.0],
                    size: [2.0 * layout.s, layout.h * layout.s]
                }
            ),
            fileg: RwLock::new(Gutter::new(
                12,
                |buffer, layout, ink| area(buffer, layout.de(), 89.0, layout.de() + 61.0, layout.s, ink),
                palette,
                |layout, backdrop| Rect {
                    backdrop,
                    border: Border::new([0.0; 4], 0.0, 0.0),
                    position: [(layout.de() + 61.0) * layout.s, 67.0 * layout.s],
                    size: [2.0 * layout.s, (layout.h - 67.0) * layout.s]
                },
                buffers::TEXT.as_bytes(),
                51.0
            )),
            // Placeholder until files can be opened
            fileh: RwLock::new(Heading::new(
                ("editor.rs", |buffer, layout, ink| area(buffer, layout.de() + 18.0, 42.0, layout.w - 18.0, layout.s, ink)),
                palette,
                ("ken/src", |buffer, layout, ink| area(buffer, layout.de() + 18.0, 21.0, layout.w - 18.0, layout.s, ink)),
                |layout, backdrop| Rect {
                    backdrop,
                    border: Border::new([0.0; 4], 0.0, 0.0),
                    position: [layout.de() * layout.s, 65.0 * layout.s],
                    size: [(layout.w - layout.de()) * layout.s, 2.0 * layout.s]
                }
            )),
            filesg: {
                let ken = ken.read().unwrap();
                let listing = ken.directories.get(&ken.active).map(|buffer| buffer.read(..).unwrap()).unwrap_or_default();
                
                RwLock::new(Gutter::new(
                    1,
                    |buffer, layout, ink| area(buffer, 0.0, 89.0, 38.0, layout.s, ink),
                    palette,
                    |layout, backdrop| Rect {
                        backdrop,
                        border: Border::new([0.0; 4], 0.0, 0.0),
                        position: [38.0 * layout.s, 67.0 * layout.s],
                        size: [2.0 * layout.s, (layout.h - 67.0) * layout.s]
                    },
                    listing,
                    28.0
                ))
            },
            filesh: {
                let ken = ken.read().unwrap();

                let folder = ken.active.file_name().map(|name| format!("{}/", name.display())).unwrap_or_default();
                let parent = ken.active.parent().map(|parent| {
                    if let Some(home) = home_dir() && let Ok(rest) = parent.strip_prefix(home) {
                        format!("~{}{}", MAIN_SEPARATOR, rest.display())
                    }
                    else {
                        parent.to_string_lossy().into_owned()
                    }
                }).unwrap_or_default();

                RwLock::new(Heading::new(
                    (&folder, |buffer, layout, ink| area(buffer, 18.0, 42.0, layout.fw - 18.0, layout.s, ink)),
                    palette,
                    (&parent, |buffer, layout, ink| area(buffer, 18.0, 21.0, layout.fw - 18.0, layout.s, ink)),
                    |layout, backdrop| Rect {
                        backdrop,
                        border: Border::new([0.0; 4], 0.0, 0.0),
                        position: [0.0, 65.0 * layout.s],
                        size: [layout.fw * layout.s, 2.0 * layout.s]
                    }
                ))
            },
            fp: |layout, palette| Rect {
                backdrop: palette.shell.rect(),
                border: Border::new([0.0; 4], 0.0, 0.0),
                position: [0.0, 0.0],
                size: [layout.fw * layout.s, layout.h * layout.s]
            },
            hits: RwLock::new(Hits::default()),
            ken,
            layout: RwLock::new(Layout::default())
        });
        
        editor.prepare(Layout::new(focused, 308.0, scale, size), *palette, resources).await?;
        
        resources.render(palette)?;
        
        Ok(Self::Editor(editor))
    }
}