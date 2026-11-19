mod complete;
use complete::complete;

mod init;
use init::init;

mod page;
use page::Page;

mod resources;
use resources::{Gpu, Resources, Text};

use std::sync::{Arc, RwLock};

use carreau::Carreau;
use tokio::task::spawn_blocking;
use wgpu::{Surface, SurfaceConfiguration};
use winit::window::Window;

use crate::{Error, resources::{Gpu as AppGpu, Text as AppText}};

pub(super) struct Screen {
    pub(super) page: Arc<Page>,
    pub(super) resources: Arc<Resources>,
    pub(super) surface: (Surface<'static>, SurfaceConfiguration),
    pub(super) window: Arc<Window>
}

impl Screen {
    pub(super) async fn new(
        gpu: Arc<AppGpu>,
        surface: Surface<'static>,
        text: Arc<AppText>,
        window: Arc<Window>
    ) -> Result<Self, Error> {
        let mg = gpu.clone();
        let mw = window.clone();
        let init = spawn_blocking(|| init(mg, surface, mw));

        let (cache, viewport) = Text::init(&gpu.device);

        let (suc, surface) = init.await??;

        let tf = suc.format;

        let md = gpu.device.clone();
        let mw = window.clone();
        let complete = spawn_blocking(|| complete(md, suc, surface, mw));

        let md = gpu.device.clone();
        let carreau = spawn_blocking(move || Carreau::new(3, &md, tf));

        let fs = text.fs.clone();
        let swc = text.sc.clone();
        let md = gpu.device.clone();
        let mq = gpu.queue.clone();
        let text = spawn_blocking(move || Text::complete(cache, fs, md, mq, swc, tf, viewport));

        let (surface, tv) = complete.await?;

        let resources = Arc::new(Resources {
            carreau: RwLock::new(carreau.await?),
            gpu: Gpu {
                device: gpu.device.clone(),
                queue: gpu.queue.clone()
            },
            text: text.await?,
            tv
        });

        let page = Arc::new(Page::landing(&resources, window.inner_size()).await?);

        Ok(Self {
            page,
            resources,
            surface,
            window
        })
    }
}
