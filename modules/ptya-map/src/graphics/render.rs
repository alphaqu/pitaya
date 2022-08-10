use std::rc::Rc;
use anyways::ext::AuditExt;
use glium::backend::Context;
use glium::{implement_vertex, IndexBuffer, Program, VertexBuffer};
use glium::program::ProgramCreationInput::SourceCode;
use anyways::Result;

#[derive(Copy, Clone)]
pub struct MapVertex {
	pub pos: [f32; 2],
}

// you must pass the list of members to the macro
implement_vertex!(MapVertex, pos);



// Components
// Polygon:
// for tile {
//      color (buffer)
//      stoke
// }

pub struct Renderer {
	program: Program
}

impl Renderer {
	pub fn new(ctx: &Rc<Context>) -> Result<Renderer> {
		Ok(Renderer {
			program: Program::new(ctx, SourceCode {
				vertex_shader: include_str!("./shader/vertex_fill.glsl"),
				tessellation_control_shader: None,
				tessellation_evaluation_shader: None,
				geometry_shader: None,
				fragment_shader:  include_str!("./shader/fragment_fill.glsl"),
				transform_feedback_varyings: None,
				outputs_srgb: false,
				uses_point_size: false
			}).wrap_err("Failed to create program")?
		})
	}
	
	
	pub fn draw(&self, index: Vec<u32>, vertex: Vec<MapVertex>) {
		
	}
}


pub struct MapMesh {
	pub vertex: VertexBuffer<MapVertex>,
	pub index: IndexBuffer<u32>,
}