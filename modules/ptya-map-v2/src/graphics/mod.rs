use crate::TilePosition;
use atlas::tile::TileMesh;
use fxhash::FxHashMap;
use glium::backend::Context;
use glium::framebuffer::SimpleFrameBuffer;
use glium::program::SourceCode;
use glium::{DrawParameters, Program, Surface, uniform};
use std::rc::Rc;
use anyways::ext::AuditExt;
use log::trace;
use crate::viewport::Viewport;

pub struct MapGraphics {
    program: Program,
    tiles: FxHashMap<TilePosition, TileMesh>,
}

impl MapGraphics {
    pub fn new(ctx: &Rc<Context>) -> anyways::Result<MapGraphics> {
        Ok(MapGraphics {
            program: Program::new(
                ctx,
                SourceCode {
                    vertex_shader: include_str!("./shader/vertex_fill.glsl"),
                    tessellation_control_shader: None,
                    tessellation_evaluation_shader: None,
                    geometry_shader: None,
                    fragment_shader: include_str!("./shader/fragment_fill.glsl"),
                },
            )?,
            tiles: Default::default(),
        })
    }

    pub fn contains_tile(&self, pos: TilePosition) -> bool {
        self.tiles.contains_key(&pos)
    }

	pub fn add_tile(&mut self, pos: TilePosition, mesh: TileMesh) {
		self.tiles.insert(pos, mesh);
	}

	/// Draws a tile on the screen.
	/// 
	/// 
    pub fn draw_tile(&mut self, framebuffer: &mut SimpleFrameBuffer, viewport: &Viewport, pos: TilePosition) -> anyways::Result<()>  {
	    if let Some(mesh) = self.tiles.get_mut(&pos) {
		    let num_tiles = viewport.get_num_tiles();

		    let tile_rect = pos.get_rect();
		    let tile_size = tile_rect.size();
		    let tile_view_size = tile_size / viewport.view.size();

		    let res_height = viewport.resolution.y().cast::<f32>().unwrap();
		    let pixel_scale = ((viewport.view.size().y() / tile_size.y()).any_unit() / res_height).val();
		    
		    let delta_world_pos = tile_rect.origin() - viewport.view.min();
		    let delta_view_pos = delta_world_pos / (viewport.view.size());
		    let delta_tile_pos = delta_world_pos * num_tiles;


	    //     let viewer_scale = viewer.get_scale();
	    // 	        let scale = pos.get_scale() as f32;
	    // 	        let x = (viewer.x * scale) - (pos.x as f32);
	    // 	        let y = (viewer.y * scale) - (pos.y as f32);
	    // 		    trace!("Update tile");
	        mesh.update(viewport.zoom, pixel_scale);
            framebuffer.draw(
                &mesh.vertices,
                &mesh.indices,
                &self.program,
                &uniform! {
	                scale: [*tile_view_size.x(), *tile_view_size.y()],
	                pos: [*delta_tile_pos.x(), *delta_tile_pos.y()],
                },
	            &DrawParameters {
		            backface_culling: glium::BackfaceCullingMode::CullingDisabled,
		           //scissor: Some(glium::Rect {
			       //    left: (resolution.width as f32  * delta_view_pos.x)as u32,
			       //    bottom: (resolution.height as f32  * delta_view_pos.y)as u32,
			       //    width: (resolution.width as f32 * tile_view_size.x) as u32,
			       //    height: (resolution.height as f32 * tile_view_size.y) as u32
		           //}),
		            ..DrawParameters::default()
	            }
            ).wrap_err("Failed to render tile")?;
        }

	    Ok(())
    }

	pub fn clear(&mut self, viewport: &Viewport) {
		self.tiles.drain_filter(|pos, _| {
			let rect = pos.get_rect();
			let x = !viewport.view.overlap_rect(rect);
			if x {
				trace!("Removed {pos:?}");
			}
			x
		});
	}
}