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
use ken::{Ken, buffers::File};
use winit::dpi::PhysicalSize;

use crate::{Error, screen::{Page, Resources, page::Palette}};

type Area = fn(&Buffer, Layout, Color, f32) -> TextArea<'_>;

pub(crate) struct Editor {
    divider: Divider,
    file: RwLock<Option<Pane>>,
    filesystem: RwLock<Option<Pane>>,
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
        // Set the given path as the active directory (which doubles as a boolean)
        let focused = {
            ken.write().unwrap().directories.active = path.clone();

            path.is_none()
        };
        
        let editor = Arc::new(Editor {
            divider: divider(),
            file: if focused {
                let mut ken = ken.write().unwrap();
                
                let key = ken.files.ephemeral.insert();
                ken.files.active = Some(File::Ephemeral(key));
                
                RwLock::new(Some(file("untitled", palette, "", "")))
            }
            else {
                RwLock::new(None)
            },
            filesystem: if !focused {
                // TODO: Have to handle the Result and Option (by switching to the existing tab) here
                ken.write().unwrap().directories.insert(path.unwrap()).unwrap();
                
                let directories = &ken.read().unwrap().directories;
                RwLock::new(Some(filesystem(directories, palette)))
            }
            else {
                RwLock::new(None)
            },
            ken,
            layout: RwLock::new(Layout::default())
        });
        
        editor.prepare(Layout::new(308.0, scale, size), *palette, resources).await?;
        
        resources.render(palette)?;
        
        Ok(Self::Editor(editor))
    }
}