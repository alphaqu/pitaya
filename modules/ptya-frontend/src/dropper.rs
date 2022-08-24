use crate::content::NewAppLocation;
use egui::epaint::ahash::AHashMap;
use egui::{Color32, Id, LayerId, Order, Pos2, Rect, Rounding, Stroke, Vec2};
use log::debug;
use ptya_core::animation::{Animation, Easing, Lerp};
use ptya_core::app::AppId;
use ptya_core::ui::ROUNDING;
use ptya_core::System;
use std::collections::hash_map::Entry;

pub struct AppDropper {
	pub pos: Pos2,
	pub id: AppId,
	pub dropped: bool,
	pub just_dropped: Option<NewAppLocation>,
	placements: AHashMap<NewAppLocation, Placement>,
}

impl AppDropper {
	pub fn new(id: AppId) -> AppDropper {
		debug!("Started app dropping of {id:?}");
		AppDropper {
			pos: Default::default(),
			id,
			dropped: false,
			just_dropped: None,
			placements: AHashMap::new(),
		}
	}

	pub fn add_placement(&mut self, placement: Placement) {
		if !self.dropped {
			match self.placements.entry(placement.location) {
				Entry::Occupied(mut occupied) => {
					*occupied.get_mut() = placement;
				}
				Entry::Vacant(vacant) => {
					vacant.insert(placement);
				}
			}
		}
	}

	pub fn tick(&mut self, sys: &System) {
		if !self.dropped {
			for (_, placement) in &mut self.placements {
				placement.keep = false;
			}

			let input = sys.egui_ctx.input();
			if let Some(value) = input.pointer.interact_pos() {
				self.pos = value;
			}

			if !input.pointer.primary_down() {
				debug!("Dropped {:?}", self.id);
				self.dropped = true;
				for (location, placement) in &self.placements {
					if placement.rect.contains(self.pos) {
						self.just_dropped = Some(*location);
					}
				}
			} else {
				self.placements
					.drain_filter(|_, placement| placement.get_animation(sys).get_value() == 0.0);
			}
		}
	}

	pub fn finish(&mut self, sys: &System) -> bool {
		let painter = sys
			.egui_ctx
			.layer_painter(LayerId::new(Order::Tooltip, Id::null()));

		painter.circle(self.pos, 5.0, Color32::DEBUG_COLOR, Stroke::none());
		// Drain placements that have not been called this tick
		//self.placements.drain_filter(|v, placement| !placement.keep);

		let mut finished = self.dropped;
		for (_, placement) in &self.placements {
			let color = sys.color.theme().tertiary.color;

			let v = placement
				.get_animation(sys)
				.redirect(placement.keep as u8 as f32)
				.set_easing(Easing::EaseInOut)
				.get_value();

			if v != 0.0 {
				finished = false;
			}

			let hover = placement
				.get_hover_animation(sys)
				.redirect(placement.hovered as u8 as f32)
				.set_easing(Easing::EaseInOut)
				.get_value()
				.max(1.0 - v);

			painter.rect(
				Rect::lerp_static(&placement.from, &placement.rect, v),
				ROUNDING,
				color.linear_multiply((0.05 + (0.05 * hover)) * v),
				Stroke::new((2.5 + (2.5 * hover)) * v, color),
			)
		}

		self.just_dropped = None;
		//self.placements.clear();

		finished
	}
}

pub struct Placement {
	from: Rect,
	rect: Rect,
	hovered: bool,
	keep: bool,
	location: NewAppLocation,
}

impl Placement {
	pub fn new(
		from: Rect,
		rect: Rect,
		hovered: bool,
		location: impl Into<NewAppLocation>,
	) -> Placement {
		Placement {
			from,
			rect,
			hovered,
			keep: true,
			location: location.into(),
		}
	}

	fn id(&self) -> egui::Id {
		Id::new(self.location)
	}

	fn get_animation(&self, sys: &System) -> Animation<f32> {
		sys.animation.get::<f32>(self.id())
	}

	fn get_hover_animation(&self, sys: &System) -> Animation<f32> {
		sys.animation.get::<f32>(self.id().with("hover"))
	}
}
