use crate::layer::LayerManager;
use crate::style::MapStyle;
use crate::tile::{TileData, TileMesh, TileMeshBuilder};
use fxhash::FxHashMap;
use glium::backend::Context;
use std::rc::Rc;
use glium::framebuffer::SimpleFrameBuffer;
use glium::Surface;
use mathie::{Rect, Vec2D};

pub mod feature;
pub mod geometry;
pub mod layer;
pub mod predicate;
pub mod style;
pub mod tile;
pub mod types;
mod tessellation;

pub struct Atlas {
    pub layer_managers: FxHashMap<String, LayerManager>,
}

impl Atlas {
    pub fn new(style: MapStyle) -> Atlas {
        Atlas {
            layer_managers: style
                .layers
                .into_iter()
                .map(|(name, style)| (name, LayerManager { style }))
                .collect(),
        }
    }

    pub fn compile(&self, tile: TileData, viewport: Rect<f32>, resolution: Vec2D<u32>, zoom: u8) -> TileMeshBuilder {
        let tile_scale = 2u32.pow(zoom as u32) as f32;
        let height = viewport.size().x();
        let pixel_scale = (height / (1.0 / tile_scale)) / (resolution.y() as f32);

        let mut builder = TileMeshBuilder::new();
        for layer in tile.layers {
            if let Some(manager) = self.layer_managers.get(&layer.name) {
                manager.compile(layer, pixel_scale, zoom, &mut builder);
            }
        }
        builder
    }
}
