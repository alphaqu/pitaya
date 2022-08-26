mod color;

use crate::style::color::MapColorManager;
use egui::{Align2, Color32, Context, FontId, Pos2, ProgressBar, Rgba};
use map_renderer::data::FeatureData;
use map_renderer::geometry::GeometryData;
use map_renderer::style::fill::FillStyle;
use map_renderer::style::{Exponential, Linear, StyleBuilder, StyleHandler, Styler};
use ptya_core::animation::Lerp;
use ptya_core::color::{ColorState, ColorTag, Theme};
use std::ops::Range;

pub struct MapStyler {
	ctx: Context,
	color: MapColorManager,
}

impl MapStyler {
	pub fn new(ctx: Context) -> MapStyler {
		MapStyler {
			ctx,
			color: Default::default(),
		}
	}
	pub fn update_theme(&mut self, theme: &Theme) {
		self.color = MapColorManager::new(theme);
	}

	pub fn road<S: StyleHandler>(&self, mut builder: StyleBuilder<S>) -> Option<()> {
		match builder.field("class")?.to_str()? {
			"motorway" | "trunk" => {
				// Exp(1.5, 5.0..18.0, 0.75..32.0)
				builder.stroke(
					self.color.medium_road,
					Exponential {
						base: 1.5,
						stops: [(5.0, 0.75), (18.0, 32.0)],
					},
				)
			}
			"primary" => builder.stroke(self.color.medium_road, Exponential {
				base: 1.5,
				stops: [(5.0, 0.75), (18.0, 32.0)],
			}),
			"secondary" | "tertiary" => {
				builder.stroke(self.color.medium_road, Exponential {
					base: 1.5,
					stops: [(5.0, 0.1), (18.0, 26.0)],
				})
			}
			"street" | "street_limited" | "primary_link" => {
				builder.stroke(self.color.small_road, Exponential {
					base: 1.5,
					stops: [
						(12.0, 0.5),
						(14.0, 2.0),
						(18.0, 18.0),
					],
				})
			}
			"motorway_link" | "trunk_link" => {
				builder.stroke(self.color.small_road, Exponential {
					base: 1.5,
					stops: [
						(12.0, 0.5),
						(14.0, 2.0),
						(18.0, 18.0),
					],
				})
			}
			"construction"=> {
				builder.stroke(self.color.small_road, Exponential {
					base: 1.5,
					stops: [
						(14.0, 2.0),
						(18.0, 18.0),
					],
				})
			}
			_ => {}
		}

		Some(())
	}

	pub fn land_use<S: StyleHandler>(&self, mut builder: StyleBuilder<S>) -> Option<()> {
		match builder.field("class")?.to_str()? {
			"airport" => builder.fill(self.color.airport),
			"hospital" => builder.fill(self.color.hospital),
			"park" => builder.fill(self.color.park),
			"sand" => builder.fill(self.color.beach),
			"school" => builder.fill(self.color.school),
			_ => {}
		}

		Some(())
	}

	pub fn water<S: StyleHandler>(&self, mut builder: StyleBuilder<S>) {
		builder.fill(self.color.water);
	}

	pub fn admin<S: StyleHandler>(&self, mut builder: StyleBuilder<S>) -> Option<()> {
		match builder.field("admin_level")?.to_f64()? as i32 {
			0 => {
				builder.stroke(self.color.border_0, Linear {
					stops: [
						(3.0, 0.5),
						(10.0, 2.0),
					]
				});
			}
			1 => {
				builder.stroke(self.color.border_1, Linear {
					stops: [
						(3.0, 0.5),
						(10.0, 2.0),
					]
				});
			}
			2 => {
				//builder.stroke(self.color.border_2, (3.0..10.0, 1.0, 4.0));
			}
			_ => {}
		}

		Some(())
	}
}

impl Styler for MapStyler {
	fn visit_features<S: StyleHandler>(
		&self,
		handler: &mut S,
		layer: &str,
		zoom: f32,
		features: &[FeatureData],
	) {
		match layer {
			"admin" => {
				for feature in features {
					self.admin(StyleBuilder {
						zoom,
						feature,
						handler,
					});
				}
			}
			"road" => {
				for feature in features {
					self.road(StyleBuilder {
						zoom,
						feature,
						handler,
					});
				}
			}
			"water" => {
				for feature in features {
					self.water(StyleBuilder {
						zoom,
						feature,
						handler,
					});
				}
			}
			"landuse" => {
				for feature in features {
					self.land_use(StyleBuilder {
						zoom,
						feature,
						handler,
					});
				}
			}
			_ => {}
		}
	}

	fn get_z_index(&self, layer: &str) -> f32 {
		match layer {
			"landuse" => 0.0,
			"road" => 1.0,
			"water" => -1.0,
			_ => 0.0,
		}
	}
}

fn exp(zoom: f32, base: f32, z: Range<f32>, v: Range<f32>) -> f32 {
	if zoom <= z.start {
		v.start
	} else if zoom >= z.end {
		v.end
	} else {
		let t = (zoom - z.start) / (z.end - z.start);
		f32::lerp_static(&v.start, &v.end, t.powf(base))
	}
}
