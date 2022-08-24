use egui::Rgba;
use map_renderer::data::{FeatureData, Value};
use map_renderer::geometry::GeometryData;
use map_renderer::style::fill::FillStyle;
use map_renderer::style::stroke::StrokeStyle;
use map_renderer::style::{StyleHandler, Styler};
use ptya_core::animation::Lerp;
use ptya_core::color::{ColorState, ColorTag, Theme};
use std::ops::Range;

pub struct MapStyler {
	pub theme: Theme,
	pub level: f32,
}

impl MapStyler {
	fn get_state(&self, value: f32) -> ColorState {
		ColorState {
			level: self.level + value,
			theme: &self.theme,
		}
	}
	fn bg(&self, value: f32, ty: ColorTag, opacity: f32) -> [f32; 4] {
		let color32 = ColorState {
			level: self.level + value,
			theme: &self.theme,
		}
		.tag_bg(ty);
		let original = ColorState {
			level: 1.0,
			theme: &self.theme,
		}
		.bg();

		let road_color: Rgba = Rgba::from(original.lerp(&color32, opacity));
		road_color.to_array()
	}

	pub fn road<S: StyleHandler>(
		&self,
		zoom: f32,
		handler: &mut S,
		feature: &FeatureData,
	) -> Option<()> {
		let style = match feature.fields.get("class")?.to_str()? {
			"motorway" | "primary" | "trunk" => StrokeStyle {
				color: self.bg(8.0, ColorTag::Primary, 1.0),
				width: exp(zoom - 1.0, 1.5, 5.0..18.0, 0.75..16.0),
			},
			"secondary" | "tertiary" => StrokeStyle {
				color: self.bg(6.0, ColorTag::Primary, 1.0),
				width: exp(zoom - 1.0, 1.5, 5.0..18.0, 0.75..16.0),
			},
			"street" | "street_limited" | "primary_link" => StrokeStyle {
				color: self.bg(4.0, ColorTag::Primary, 1.0),
				width: exp(zoom - 1.0, 1.5, 12.0..18.0, 1.0..12.0),
			},
			_ => {
				return Some(());
			}
		};

		self.render_stroke(handler, feature, style)
	}

	pub fn land_use<S: StyleHandler>(
		&self,
		zoom: f32,
		handler: &mut S,
		feature: &FeatureData,
	) -> Option<()> {
		let atmosphere_opacity = 1.0 - ((zoom - 10.0) / 3.0).clamp(0.0, 1.0);
		const LIGHT_SHADE: f32 = 2.0;
		const NORMAL_SHADE: f32 = 3.5
		;
		const HEAVY_SHADE: f32 = 5.0;
		let style = match feature.fields.get("class")?.to_str()? {
			//"agriculture" => FillStyle {
			//	color: self.bg(LIGHT_SHADE, ColorTag::Yellow),
			//},
			"airport" => FillStyle {
				color: self.bg(0.5, ColorTag::Secondary, 1.0),
			},
			"cemetery" => FillStyle {
				color: self.bg(NORMAL_SHADE, ColorTag::Primary, 1.0),
			},
			"commercial_area" => FillStyle {
				color: self.bg(LIGHT_SHADE, ColorTag::Blue, 1.0),
			},
			"facility" => FillStyle {
				color: [1.0, 0.0, 0.0, 1.0],
			},
			"glacier" => FillStyle {
				color: self.bg(HEAVY_SHADE, ColorTag::Primary, 1.0),
			},
			"grass" => FillStyle {
				color: self.bg(LIGHT_SHADE, ColorTag::Green, atmosphere_opacity),
			},
			"hospital" => FillStyle {
				color: self.bg(HEAVY_SHADE, ColorTag::Red, 1.0),
			},
			"industrial" => FillStyle {
				color: self.bg(NORMAL_SHADE, ColorTag::Primary, 1.0),
			},
			"park" => FillStyle {
				color: self.bg(LIGHT_SHADE, ColorTag::Green, 1.0),
			},
			"parking" => FillStyle {
				color: self.bg(LIGHT_SHADE, ColorTag::Primary, 1.0),
			},
			"piste" => FillStyle {
				color: self.bg(HEAVY_SHADE, ColorTag::Primary, 1.0),
			},
			"pitch" => FillStyle {
				color: self.bg(NORMAL_SHADE, ColorTag::Green, 1.0),
			},
			//"residential" => FillStyle {
			//    color:  [1.0, 0.0, 0.0, 1.0],
			//},
			"rock" => FillStyle {
				color: self.bg(NORMAL_SHADE, ColorTag::Primary, 1.0),
			},
			"sand" => FillStyle {
				color: self.bg(NORMAL_SHADE, ColorTag::Yellow, 1.0),
			},
			"school" => FillStyle {
				color: self.bg(HEAVY_SHADE, ColorTag::Orange, 1.0),
			},
			"scrub" => FillStyle {
				color: self.bg(LIGHT_SHADE, ColorTag::Yellow, atmosphere_opacity),
			},
			"wood" => FillStyle {
				color: self.bg(LIGHT_SHADE, ColorTag::Green, atmosphere_opacity),
			},
			_ => {
				return Some(());
			}
		};

		self.render_fill(handler, feature, style)
	}

	pub fn water<S: StyleHandler>(&self, zoom: f32, handler: &mut S, feature: &FeatureData) {
		match &feature.geometry {
			GeometryData::Path(path) => {
				for path in &path.paths {
					// handler.submit(path, StrokeStyle::new([255, 0, 0], 1.0));
				}
			}
			GeometryData::Fill(fill) => {
				for polygon in &fill.polygons {
					handler.submit(
						polygon,
						FillStyle {
							color: self.bg(0.0, ColorTag::Blue, 1.0),
						},
					);
				}
			}
		}
	}

	pub fn render_stroke<S: StyleHandler>(
		&self,
		handler: &mut S,
		feature: &FeatureData,
		stroke: StrokeStyle,
	) -> Option<()> {
		match &feature.geometry {
			GeometryData::Path(path) => {
				for path in &path.paths {
					handler.submit(path, stroke);
				}
			}
			GeometryData::Fill(_) => {}
		}
		Some(())
	}

	pub fn render_fill<S: StyleHandler>(
		&self,
		handler: &mut S,
		feature: &FeatureData,
		style: FillStyle,
	) -> Option<()> {
		match &feature.geometry {
			GeometryData::Path(path) => {}
			GeometryData::Fill(fill) => {
				for polygon in &fill.polygons {
					handler.submit(polygon, style);
				}
			}
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
			"road" => {
				for feature in features {
					self.road(zoom, handler, feature);
				}
			}
			"water" => {
				for feature in features {
					self.water(zoom, handler, feature);
				}
			}
			"landuse" => {
				for feature in features {
					self.land_use(zoom, handler, feature);
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
