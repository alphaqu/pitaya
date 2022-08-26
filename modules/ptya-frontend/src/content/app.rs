use crate::AppDropper;
use egui::{pos2, Id, LayerId, Mesh, Order, Rect, Rgba, Rounding, Sense, Vec2};
use epaint::{Color32, RectShape, Tessellator};
use glium::framebuffer::SimpleFrameBuffer;
use glium::texture::{MipmapsOption, SrgbFormat, SrgbTexture2d};
use glium::Surface;
use ptya_core::animation::{Animation, AnimationImpl, Easing, Lerp};
use ptya_core::app::{App, AppContainer, AppId};
use ptya_core::color::ColorTag;
use ptya_core::ui::{Pui, INTERACTIVE_SIZE, ROUNDING, SPACING_SIZE};
use ptya_core::System;
use std::rc::Rc;
use log::warn;

pub struct AppPanel {
	id: AppId,
	rect: Rect,
	animation: Option<Id>,
}

impl AppPanel {
	pub fn new(sys: &System, id: AppId, from: Rect) -> AppPanel {
		AppPanel {
			id,
			rect: from,
			animation: None,
		}
	}

	fn get_ani(ui: &mut Pui, rect: Rect, id: Id) -> Animation<Rect> {
		ui.sys
			.animation
			.get_or::<Rect>(id, || AnimationImpl::simple(rect))
	}

	pub fn set_rect(&mut self, ui: &mut Pui, rect: Rect) {
		if let Some(id) = self.animation {
			if Self::get_ani(ui, rect, id).is_finished() {
				self.animation = None;
			}
		}

		if self.rect != rect && self.animation.is_none() {
			let id = self.id.egui_id().with("rect_animation");
			Self::get_ani(ui, rect, id)
				.set_from(self.rect)
				.set_to(rect)
				.set_easing(Easing::EaseInOut)
				.begin();

			self.animation = Some(id);
			self.rect = rect;
		}
	}

	pub fn get_rect(&mut self, ui: &mut Pui) -> Rect {
		if let Some(id) = self.animation {
			Self::get_ani(ui, self.rect, id).get_value()
		} else {
			self.rect
		}
	}

	pub fn draw(
		&mut self,
		ui: &mut Pui,
		dropper: &mut Option<AppDropper>,
	) -> Result<(), AppResponse> {
		let mut ui = ui.ascend(1.0);
		if let Some(container) = ui.sys.app.apps().get_mut(&self.id) {
			let rect = self.get_rect(&mut ui);

			ui.allocate_ui_at_rect(rect, |eui| {
				eui.set_clip_rect(rect);
				eui.set_min_size(rect.size());

				self.draw_app(eui, container, dropper)
			})
			.inner?;

			//let mut eui = ui.child_ui_with_id_source(rect, Layout::default(), self.id.egui_id().with("ui"));
			//eui.set_clip_rect(rect);
		}

		Ok(())
	}

	fn draw_app(
		&mut self,
		ui: &mut Pui,
		app: &mut AppContainer,
		dropper: &mut Option<AppDropper>,
	) -> Result<(), AppResponse> {
		let rect = ui.max_rect();
		self.draw_window_descriptor(ui, rect, dropper)?;

		let ppp = ui.ctx().pixels_per_point();
		let width = (rect.width() ) as u32;
		let height = (rect.height()) as u32;

		// TODO redraw on actual change
		if app.framebuffer.width() != width || app.framebuffer.height() != height {
			app.framebuffer =
				Rc::new(SrgbTexture2d::empty(&ui.sys().gl_ctx, width, height).unwrap());
			app.dirty = true;
		}

		ui.painter().rect_filled(rect, ROUNDING, ui.color().bg());

		if let Some(id) = app.id {
			let ctx = ui.ctx();
			let pixels_per_point = ctx.pixels_per_point();
			let options = *ctx.tessellation_options();
			let texture_atlas = ctx.fonts().texture_atlas();
			let font_tex_size = texture_atlas.lock().size();
			let prepared_discs = texture_atlas.lock().prepared_discs();

			let mut tessellator =
				Tessellator::new(pixels_per_point, options, font_tex_size, prepared_discs);
			let mut mesh = Mesh::with_texture(id);
			tessellator.tessellate_rect(
				&RectShape {
					rect,
					rounding: ROUNDING,
					fill: Color32::WHITE,
					stroke: Default::default(),
				},
				&mut mesh,
			);

			for vertex in &mut mesh.vertices {
				let pos = ((vertex.pos - rect.min) / rect.size()).to_pos2();
				vertex.uv = pos2(pos.x, 1.0 - pos.y);
			}

			ui.painter().add(mesh);
		}


		let mut fb = SimpleFrameBuffer::new(&ui.sys().gl_ctx, &*app.framebuffer).unwrap();
		app.app.tick(ui, &mut fb);

		Ok(())
	}

	fn draw_window_descriptor(
		&self,
		ui: &mut Pui,
		app_rect: Rect,
		dropper: &mut Option<AppDropper>,
	) -> Result<(), AppResponse> {
		let id = ui.id().with("window");

		let mut animation = ui.sys().animation.get(id);
		if dropper.is_some() {
			animation.redirect(0.0);
		} else {
			animation.redirect(1.0);
		}
		let v = animation.get_value();
		let size = Vec2::new((INTERACTIVE_SIZE * 0.75) * v, (INTERACTIVE_SIZE * 0.75) * v);
		let rect = Rect::from_min_size(app_rect.right_top() - Vec2::new(size.x, 0.0), size);

		let color = ui.color().ascend(2.0).tag_bg(ColorTag::Secondary);
		let response = ui.interact(rect, id, Sense::click_and_drag());
		if dropper.is_none() && response.drag_started() {
			return Err(AppResponse::Move);
		}

		ui.painter().rect_filled(
			rect,
			Rounding::none().lerp(
				&Rounding {
					nw: 0.0,
					se: 0.0,
					..ROUNDING
				},
				v,
			),
			color,
		);
		Ok(())
	}

	pub fn id(&self) -> &AppId {
		&self.id
	}
}

pub enum AppResponse {
	Move,
}
