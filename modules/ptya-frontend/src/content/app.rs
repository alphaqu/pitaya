use egui::epaint::ClippedShape;
use egui::{Align, Area, Color32, Id, Layout, Order, PaintCallback, Rect, Rgba, Vec2};
use glium::framebuffer::SimpleFrameBuffer;
use glium::texture::{MipmapsOption, SrgbFormat, SrgbTexture2d};
use glium::Surface;
use ptya_core::animation::{Animation, AnimationImpl, Easing};
use ptya_core::app::{App, AppId};
use ptya_core::ui::components::ProgressSpinner;
use ptya_core::ui::{Pui, ROUNDING, SPACING_SIZE};
use ptya_core::System;
use std::rc::Rc;

pub struct AppPanel {
	id: AppId,
	rect: Rect,
	animation: Option<Id>,

	// App Drawing
	buffer: Rc<SrgbTexture2d>,
}

impl AppPanel {
	pub fn new(sys: &System, id: AppId, from: Rect) -> AppPanel {
		AppPanel {
			id,
			rect: from,
			animation: None,
			buffer: Rc::new(
				SrgbTexture2d::empty_with_format(
					&sys.gl_ctx,
					SrgbFormat::U8U8U8U8,
					MipmapsOption::NoMipmap,
					0,
					0,
				)
				.unwrap(),
			),
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

	pub fn draw(&mut self, ui: &mut Pui) {
		let mut ui = ui.ascend(1.0);
		if let Some(container) = ui.sys.app.apps().get_mut(&self.id) {
			let rect = self.get_rect(&mut ui);
			ui.allocate_ui_at_rect(rect, |eui | {
				eui.set_clip_rect(rect);
				self.draw_app(eui, container.app());
			});
			//let mut eui = ui.child_ui_with_id_source(rect, Layout::default(), self.id.egui_id().with("ui"));
			//eui.set_clip_rect(rect);
		}
	}

	fn draw_app(&mut self, ui: &mut Pui, app: &mut dyn App) {
		let rect = ui.max_rect();
		let ppp = ui.ctx().pixels_per_point();

		let width = (rect.width() * ppp) as u32;
		let height = (rect.height() * ppp) as u32;

		// TODO redraw on actual change
		let mut redraw = true;
		if self.buffer.width() != width || self.buffer.height() != height {
			self.buffer = Rc::new(SrgbTexture2d::empty(&ui.sys().gl_ctx, width, height).unwrap());
			redraw = true;
		}

		if redraw {
			let shape = {
				let mut fb = SimpleFrameBuffer::new(&ui.sys().gl_ctx, &*self.buffer).unwrap();
				let bg: Rgba = ui.color().bg().into();
				fb.clear_color(bg.r(), bg.b(), bg.g(), 1.0);

				let id = self.id.egui_id().with("app_window");
				let area = Area::new(id)
					.order(Order::Middle)
					.fixed_pos(rect.min)
					.drag_bounds(rect.shrink(SPACING_SIZE)).interactable(true);

				let layer_id = area.layer();

				let rect1 = area.show(ui.ctx(), |app_ui| {
					app_ui.set_min_size(rect.size());
					app_ui.set_clip_rect(app_ui.clip_rect().expand(SPACING_SIZE));
					let mut pui = ui.child(app_ui, 0.0, None);
					app.tick(&mut pui, &mut fb);
				}).response.rect;
				//ui.ctx().debug_painter().debug_rect(rect1, Color32::TEMPORARY_COLOR, "Window");


				let shape: Vec<ClippedShape> = ui
					.ctx()
					.layer_painter(layer_id)
					.paint_list()
					.0
					.drain(..)
					.collect();
				shape
			};

			ui.painter().add(PaintCallback {
				rect,
				callback: Rc::new((self.buffer.clone(), shape)),
			});
		}

		ui.painter().add(PaintCallback {
			rect,
			callback: Rc::new((self.buffer.clone(), ROUNDING)),
		});
	}
	pub fn id(&self) -> &AppId {
		&self.id
	}
}
