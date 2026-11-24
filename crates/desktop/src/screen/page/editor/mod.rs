mod buffers;

pub(super) mod components;
use components::{Divider, Pane};

mod instances;
use instances::{divider, file, filesystem};

mod layout;
pub(crate) use layout::Layout;

mod methods;

mod render;

use std::{path::PathBuf, sync::{Arc, RwLock}};

use glyphon::{Buffer, Color, TextArea};
use ken::Ken;
use winit::dpi::PhysicalSize;

use crate::{Error, screen::{Page, Resources, page::Palette}};

type Area = fn(&Buffer, Layout, Color) -> TextArea<'_>;

pub(crate) struct Editor {
    divider: Divider,
    file: RwLock<Option<Pane>>,
    filesystem: RwLock<Pane>,
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
            divider: divider(),
            file: if focused {
                RwLock::new(Some(file(("untitled", ""), palette)))
            }
            else {
                RwLock::new(None)
            },
            filesystem: {
                let ken = ken.read().unwrap();
                
                filesystem(&ken, palette)
            },
            ken,
            layout: RwLock::new(Layout::default())
        });
        
        editor.prepare(Layout::new(focused, 308.0, scale, size), *palette, resources).await?;
        
        resources.render(palette)?;
        
        Ok(Self::Editor(editor))
    }
}