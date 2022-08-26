use crate::unit::TileUnit;
use crate::viewport::Viewport;
use crate::{draw_debug, test_draw_tile, MapStyler, TilePosition};
use anyways::ext::AuditExt;
use egui::{Color32, Painter, Pos2, Stroke, Vec2};
use glium::backend::Context;
use glium::framebuffer::SimpleFrameBuffer;
use glium::program::SourceCode;
use glium::{uniform, DrawParameters, Program, Surface};
use log::{info, trace};
use mathie::{Rect, Vec2D};
use std::rc::Rc;
use ahash::{AHashMap, AHashSet};
use map_renderer::mesh::Mesh;

pub struct MapGraphics {
	program: Program,
	drawn_tiles: AHashSet<TilePosition>,
	tiles: AHashMap<TilePosition, Mesh>,
}

impl MapGraphics {
	pub fn new(ctx: &Rc<Context>) -> anyways::Result<MapGraphics> {
		Ok(MapGraphics {
			program: Program::new(
				ctx,
				SourceCode {
					vertex_shader: include_str!("shader/vertex_fill.glsl"),
					tessellation_control_shader: None,
					tessellation_evaluation_shader: None,
					geometry_shader: None,
					fragment_shader: include_str!("shader/fragment_fill.glsl"),
				},
			)?,
			drawn_tiles: Default::default(),
			tiles: Default::default(),
		})
	}

	pub fn contains_tile(&self, pos: TilePosition) -> bool {
		self.tiles.contains_key(&pos)
	}

	pub fn add_tile(&mut self, pos: TilePosition, mesh: Mesh) {
		self.tiles.insert(pos, mesh);
	}

	//let tile_rect = pos.get_rect();
	// 				let tile_size = tile_rect.size();
	//
	// 				let tile_view_size = (tile_size / viewport.view.size()).any_unit();
	//
	// 				let res_height = viewport.resolution.y() as f32;
	// 				let pixel_scale = (viewport.view.size().y() / tile_size.y()) / res_height;
	//
	// 				let delta_world_pos = viewport.view.min() - tile_rect.min();
	// 				//  let delta_view_pos = delta_world_pos / (viewport.view.size());
	// 				let delta_tile_pos = delta_world_pos * num_tiles;

	//     let viewer_scale = viewer.get_scale();
	// 	        let scale = pos.get_scale() as f32;
	// 	        let x = (viewer.x * scale) - (pos.x as f32);
	// 	        let y = (viewer.y * scale) - (pos.y as f32);
	// 		    trace!("Update tile");
	//let num_tiles = viewport.get_num_tiles();

	// Tile position is 0 - 1
	// glsl code:
	// vec2 u_pos = ((a_pos + pos) * scale);

	// The position is supposed to be the tile offset.
	//	let delta_pos = (viewport.view.min() - tile_rect.min()).any_unit() * num_tiles;

	/// Draws a tile on the screen.
	///
	///
	pub fn draw_tile(
		&mut self,
		styler: &MapStyler,
		painter: &Painter,
		minimap: egui::Rect,
		screen: egui::Rect,
		framebuffer: &mut SimpleFrameBuffer,
		viewport: &Viewport,
		pos: TilePosition,
	) -> anyways::Result<()> {
		if !self.drawn_tiles.contains(&pos) {
			self.drawn_tiles.insert(pos);
			draw_debug(painter, minimap, pos.get_rect(), Color32::GREEN);
			if let Some(mesh) = self.tiles.get_mut(&pos) {
				let tile_rect = pos.get_rect();
				let view_rect = viewport.view;

				// Calculate scale
				let scale = (tile_rect.size() / view_rect.size()).any_unit();

				// Calculate pos
				let view_pos = view_rect.min().convert_u(TileUnit(pos.zoom));
				let tile_pos = tile_rect.min().convert_u(TileUnit(pos.zoom));
				let pos = (tile_pos - view_pos).any_unit();

				// Calculate pixel scale.
				// Where 1 is an entire tile.

				let view_size = Vec2D::new(
					framebuffer.get_dimensions().0 as f64,
					framebuffer.get_dimensions().1 as f64,
				);

				test_draw_tile(painter, screen, pos, scale);
				let cull = {
					let add: Rect<f64> = Rect::new([pos.x(), pos.y()], [1.0, 1.0]);
					let scaled: Rect<f64> = add * scale;
					let tile = scaled * view_size;

					mesh.update(
						styler,
						viewport.zoom as f32,
						(1.0 / tile.size().y()) as f32,
					);

					let other = Rect::new_any([0.0, 0.0], view_size);
					let out = Rect::new_any_min_max(
						Vec2D::new_any(
							tile.min().x().max(other.min().x()),
							tile.min().y().max(other.min().y()),
						),
						Vec2D::new_any(
							tile.max().x().min(other.max().x()),
							tile.max().y().min(other.max().y()),
						),
					);

					//  Self {
					//             min: self.min.max(other.min),
					//             max: self.max.min(other.max),
					//         }
					// tile.intersect(egui::Rect::from_min_size(Pos2::new(0.0, 0.0), view_size))
					out
				};
				//mesh.update(styler, viewport.zoom, 1.0 / (view_size.y as f32 * scale.y()));

				framebuffer
					.draw(
						&mesh.vertices,
						&mesh.indices,
						&self.program,
						&uniform! {
							// The scale of the tile relative to the screen
							scale: [scale.x() as f32, scale.y() as f32],
							pos: [pos.x() as f32, pos.y() as f32],
						},
						&DrawParameters {
							backface_culling: glium::BackfaceCullingMode::CullingDisabled,
							scissor: Some(glium::Rect {
								left: (cull.left() as u32),
								bottom: (view_size.y() - (cull.bottom())) as u32,
								width: cull.size().x().ceil() as u32,
								height: cull.size().y().ceil() as u32,
							}),
							..DrawParameters::default()
						},
					)
					.wrap_err("Failed to render tile")?;
			}
		}

		Ok(())
	}

	pub fn clear(&mut self, painter: &Painter, minimap: egui::Rect, viewport: &Viewport) {
		self.drawn_tiles.clear();
		self.tiles.drain_filter(|pos, _| {
			let rect = pos.get_rect();
			let x = !viewport.view.intersects_rect(rect);
			if x {
				draw_debug(painter, minimap, rect, Color32::BLUE);
			}
			x
		});
	}
}
