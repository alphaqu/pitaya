use crate::{MapStorage, Tile, TilePosition};
use anyways::Result;
use std::collections::{HashMap, HashSet};
use euclid::default::{Box2D, Point2D, Rect, Vector2D};

pub struct MapViewer {
    pub zoom: f32,
    pub x: f32,
    pub y: f32,
}

impl MapViewer {
    pub fn get_viewport(&self, aspect_ratio: f32) -> Box2D<f32> {
        let scale = self.get_scale();

        let width = (0.5 / scale) * aspect_ratio;
        let min_x = self.x - width;
        let max_x = self.x + width;

        let height = (0.5 / scale);
        let min_y = self.y - height;
        let max_y = self.y + height;

        Box2D::new(
            Point2D::new(min_x, min_y),
            Point2D::new(max_x, max_y),
        )
    }

    pub fn get_tile_viewport(&self, viewport: Box2D<f32>) -> Box2D<f32> {
        let level_scale = self.get_level_scale();
        fn calc_pos(pos: f32, scale: u32) -> f32 {
            let mut value = pos * scale as f32;
            if value < 0.0 {
                value -= 1.0;
            }
            value
        }

        Box2D::new(
            Point2D::new(calc_pos(viewport.min.x, level_scale), calc_pos(viewport.min.y, level_scale)),
            Point2D::new(calc_pos(viewport.max.x, level_scale), calc_pos(viewport.max.y, level_scale)),
        )
    }

    pub fn get_tiles<'a>(&mut self, storage: &'a mut MapStorage, tile_viewport: &Box2D<f32>) -> Result<Vec<TileRenderRequest<'a>>> {
        let level_scale = self.get_level_scale() as f32;

        // rendered-pos : not
        let mut out: HashMap<TilePosition, TilePosition> = HashMap::new();
        for y in (tile_viewport.min.y as i32)..=(tile_viewport.max.y as i32) {
            for x in (tile_viewport.min.x as i32)..=(tile_viewport.max.x as i32) {
                let x = ((x as f32 / level_scale).rem_euclid(1.0) * level_scale) as u32;
                let y = ((y as f32 / level_scale).rem_euclid(1.0) * level_scale) as u32;

                let pos = TilePosition {
                    zoom: self.zoom as u8,
                    x,
                    y,
                };

                if let Some(render_tile) = self.get_tile(pos, storage, &out)? {
                    out.insert(render_tile, pos);
                }
            }
        }

        let mut vec: Vec<TileRenderRequest> = out.into_iter().flat_map(|(render_tile_pos, cull_tile_pos )| {
            storage.get_tile(render_tile_pos).map(|render_tile| TileRenderRequest {
                render_tile,
                render_tile_pos,
                cull_tile_pos
            })
        }).collect();
        vec.sort_by(|v0, v1| v0.render_tile_pos.zoom.cmp(&v1.render_tile_pos.zoom));
        Ok(vec)
    }

    fn get_tile(
        &mut self,
        pos: TilePosition,
        storage: &mut MapStorage,
        out: &HashMap<TilePosition, TilePosition>,
    ) -> Result<Option<TilePosition>> {
        if !out.contains_key(&pos) {
            if storage.prepare_tile(pos)? {
                return Ok(Some(pos));
            } else if let Some(outer) = pos.get_outer() {
                return self.get_tile(outer, storage, out);
            }
        }
        Ok(None)
    }

    pub fn get_scale(&self) -> f32 {
        2f32.powf(self.zoom)
    }

    pub fn get_level_scale(&self) -> u32 {
        2u32.pow(self.zoom as u32)
    }
}

pub struct TileRenderRequest<'a> {
    pub render_tile: &'a Tile,
    pub render_tile_pos: TilePosition,
    pub cull_tile_pos: TilePosition,
}