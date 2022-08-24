use crate::style::StyleAllocation;
use crate::{MapVertex, Style, StyleHandler, Styler, TileData};
use glium::backend::Context;
use glium::index::PrimitiveType;
use glium::{IndexBuffer, VertexBuffer};
use std::ops::Range;
use std::rc::Rc;

pub struct MeshBuilder {
	data: TileData,
	compile: MeshCompile,
}

impl MeshBuilder {
	pub fn compile<S: Styler>(styler: &S, mut data: TileData, zoom: u8, scale: f32) -> MeshBuilder {
		let mut counter = MeshLengthCounter { length: 0 };
		data.layers.drain_filter(|layer| {
			let styles_count = counter.length;
			styler.visit_features(&mut counter, &layer.name, zoom as f32, &layer.features);
			counter.length == styles_count
		});

		let mut compile = MeshCompile {
			scale,
			indices: vec![],
			vertices: Vec::with_capacity(counter.length),
			styles: vec![],
		};

		// Order layers
		data.layers.sort_by(|v0, v1| {
			styler.get_z_index(&v0.name).total_cmp(&styler.get_z_index(&v1.name))
		});

		for data in &data.layers {
			styler.visit_features(&mut compile, &data.name, zoom as f32, &data.features);
		}

		MeshBuilder { data, compile }
	}

	pub fn build(self, ctx: &Rc<Context>) -> Mesh {
		Mesh {
			data: self.data,
			vertices: VertexBuffer::new(ctx, &self.compile.vertices).unwrap(),
			indices: IndexBuffer::immutable(
				ctx,
				PrimitiveType::TrianglesList,
				&self.compile.indices,
			)
			.unwrap(),
			vertices_data: self.compile.vertices,
			styles: self.compile.styles,
		}
	}
}

/// The ratio of changes to styles that is required to fully rewrite the entire vertices instead of going through all of the styles and updating small chunks
pub const FULL_WRITE_THRESHOLD: f32 = 0.5;

pub struct Mesh {
	pub data: TileData,

	vertices_data: Vec<MapVertex>,
	pub vertices: VertexBuffer<MapVertex>,
	pub indices: IndexBuffer<u32>,

	styles: Vec<StyleAllocation>,
}

impl Mesh {
	pub fn update<S: Styler>(&mut self, styler: &S, zoom: f32, scale: f32) {
		let styles_len = self.styles.len();

		let mut update = MeshUpdate {
			scale,
			pos: 0,
			vertices: &mut self.vertices_data,
			styles: &mut self.styles,
			changes: vec![],
		};

		for layer in &self.data.layers {
			styler.visit_features(&mut update, &layer.name, zoom, &layer.features);
		}

		let change_ratio = update.changes.len() as f32 / styles_len as f32;
		if change_ratio > FULL_WRITE_THRESHOLD {
			self.vertices.write(&self.vertices_data);
		} else {
			for range in update.changes {
				let slice = self
					.vertices
					.slice_mut(range.clone())
					.expect("Slice does not exist for this range");
				slice.write(&self.vertices_data[range]);
			}
		}
	}
}

struct MeshLengthCounter {
	length: usize,
}

impl StyleHandler for MeshLengthCounter {
	fn submit<'a, S: Style>(&'a mut self, input: impl Into<S::Input<'a>>, _: S) {
		self.length += S::get_len(input.into());
	}
}

struct MeshCompile {
	scale: f32,
	vertices: Vec<MapVertex>,
	indices: Vec<u32>,
	styles: Vec<StyleAllocation>,
}

impl StyleHandler for MeshCompile {
	fn submit<'a, S: Style>(&'a mut self, input: impl Into<S::Input<'a>>, mut style: S) {
		let start = self.vertices.len();

		style.prepare(self.scale);
		style.compile(input.into(), &mut self.vertices, &mut self.indices);

		self.styles.push(StyleAllocation {
			old_style: Box::new(style),
			vertices_range: start..self.vertices.len(),
		})
	}
}

struct MeshUpdate<'a> {
	scale: f32,
	pos: usize,
	vertices: &'a mut Vec<MapVertex>,
	styles: &'a mut [StyleAllocation],
	changes: Vec<Range<usize>>,
}

impl<'b> StyleHandler for MeshUpdate<'b> {
	fn submit<'a, S: Style>(&'a mut self, input: impl Into<S::Input<'a>>, mut style: S) {
		style.prepare(self.scale);

		let alloc = &mut self.styles[self.pos];
		self.pos += 1;
		let old_styler: &mut S = alloc
			.old_style
			.downcast_mut::<S>()
			.expect("Stylers changed with the same tile data. This is very wrong.");
		if style.needs_update(*old_styler) {
			let range = alloc.vertices_range.clone();
			let mut combined = false;
			if let Some(last) = self.changes.last_mut() {
				if last.end == range.start {
					last.end = range.end;
					combined = true;
				}
			}

			if !combined {
				self.changes.push(range.clone());
			}

			style.update(input.into(), &mut self.vertices[range], Some(*old_styler));
		}

		*old_styler = style;
	}
}
