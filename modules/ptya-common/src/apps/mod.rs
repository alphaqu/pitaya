use std::ops::{Deref, DerefMut};
use egui::{ColorImage, Context, ImageData, TextureFilter, TextureHandle};
use crate::apps::app::{EGuiApplication, AppId, AppInfo, AppInstance};
use fxhash::FxHashMap;
use image::RgbaImage;

pub mod app;


#[macro_export]
macro_rules! app_icon {
    ($file:expr $(,)?) => {
	    $crate::apps::load_app_icon(include_bytes!($file))
    };
}


pub fn load_app_icon(buf: &[u8]) -> RgbaImage {
    image::load_from_memory_with_format(
        buf,
        image::ImageFormat::Png,
    )
        .unwrap()
        .brighten(255)
        .resize(70, 70,  image::imageops::FilterType::CatmullRom)
        .to_rgba8()
}

pub struct Apps {
    pub apps: FxHashMap<AppId, AppContainer>,
}

impl Apps {
    pub fn new() -> Apps {
        Apps {
            apps: Default::default(),
        }
    }

    pub fn load_app(&mut self, app: AppContainer) {
        self.apps.insert(
            AppId {
                id: app.info.id.clone(),
            },
            app,
        );
    }


    pub fn get_mut_app(&mut self, id: &AppId) -> &mut AppContainer {
        self.apps.get_mut(id).unwrap()
    }

    pub fn get_app(&self, id: &AppId) -> &AppContainer {
        self.apps.get(id).unwrap()
    }
}

pub struct AppContainer {
    pub app: AppInstance,
    pub info: AppInfo,
    pub icon_handle: TextureHandle,
}

impl AppContainer {
    pub fn new(ctx: &Context, info: AppInfo, app: AppInstance) -> AppContainer {
        let icon_handle = ctx.load_texture(&format!("{}:icon", info.name), ColorImage::from_rgba_unmultiplied(
            [info.icon.width() as usize, info.icon.height() as usize],
            info.icon.as_ref(),
        ), TextureFilter::Nearest);

        AppContainer {
            app,
            info,
            icon_handle
        }
    }
}