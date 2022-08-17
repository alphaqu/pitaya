use atlas::data::{FeatureData, Value};
use atlas::geometry::GeometryData;
use atlas::style::fill::FillStyle;
use atlas::style::stroke::StrokeStyle;
use atlas::style::{StyleHandler, Styler};
use egui::Rgba;
use ptya_common::settings::color::{ColorSettings, ColorStyle, ColorType};
use ptya_common::ui::animation::lerp::Lerp;
use std::ops::Range;

pub struct MapStyler {
    // colors
    pub settings: ColorSettings,
}

impl MapStyler {
    fn c_bg(&self, value: f32, ty: ColorType) -> [f32; 4] {
        let road_color: Rgba = self.settings.c_bg(value, ty).into();
        road_color.to_array()
    }
    fn bg(&self, value: f32, ty: ColorType) -> [f32; 4] {
        let road_color: Rgba = self.settings.bg(value, ty).into();
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
                color: self.bg(8.0, ColorType::Primary),
                width: exp(zoom, 1.5, 5.0..18.0, 0.75..16.0),
            },
            "secondary" | "tertiary" => StrokeStyle {
                color: self.bg(6.0, ColorType::Primary),
                width: exp(zoom, 1.5, 5.0..18.0, 0.75..16.0),
            },
            "street" | "street_limited" | "primary_link" => StrokeStyle {
                color: self.bg(4.0, ColorType::Primary),
                width: exp(zoom, 1.5, 12.0..18.0, 1.0..12.0),
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

        const LIGHT_SHADE: f32 = 1.0;
        const NORMAL_SHADE: f32 = 3.0;
        const HEAVY_SHADE: f32 = 5.0;
        let style = match feature.fields.get("class")?.to_str()? {
            "agriculture" => FillStyle {
                color: self.bg(LIGHT_SHADE, ColorType::Yellow),
            },
            "airport" => FillStyle {
                color: self.bg(HEAVY_SHADE, ColorType::Blue),
            },
            "cemetery" => FillStyle {
                color: self.bg(NORMAL_SHADE, ColorType::Neutral),
            },
            "commercial_area" => FillStyle {
                color: self.bg(NORMAL_SHADE, ColorType::Blue),
            },
            "facility" => FillStyle {
                color: [1.0, 0.0, 0.0, 1.0],
            },
            "glacier" => FillStyle {
                color: self.bg(HEAVY_SHADE, ColorType::Cyan),
            },
            "grass" => FillStyle {
                color: self.bg(LIGHT_SHADE, ColorType::Green),
            },
            "hospital" => FillStyle {
                color: self.bg(HEAVY_SHADE, ColorType::Red),
            },
            "industrial" => FillStyle {
                color: self.bg(NORMAL_SHADE, ColorType::Neutral),
            },
            "park" => FillStyle {
                color: self.bg(NORMAL_SHADE, ColorType::Green),
            },
            "parking" => FillStyle {
                color: self.bg(LIGHT_SHADE, ColorType::Neutral),
            },
            "piste" => FillStyle {
                color: self.bg(HEAVY_SHADE, ColorType::Neutral),
            },
            "pitch" => FillStyle {
                color: self.bg(HEAVY_SHADE, ColorType::Green),
            },
            //"residential" => FillStyle {
            //    color:  [1.0, 0.0, 0.0, 1.0],
            //},
            "rock" => FillStyle {
                color: self.bg(HEAVY_SHADE, ColorType::Neutral),
            },
            "sand" => FillStyle {
                color: self.bg(LIGHT_SHADE, ColorType::Yellow),
            },
            "school" => FillStyle {
                color: self.bg(HEAVY_SHADE, ColorType::Orange),
            },
            "scrub" => FillStyle {
                color: self.bg(LIGHT_SHADE, ColorType::Yellow),
            },
            "wood" => FillStyle {
                color: self.bg(LIGHT_SHADE, ColorType::Yellow),
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
                            color: self.bg(-1.0, ColorType::Blue),
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
