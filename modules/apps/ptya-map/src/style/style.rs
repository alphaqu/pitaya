use crate::graphics::painter::Tessellator;
use crate::{Color32, Stroke, Vec2};
use egui::Pos2;
use lyon_tessellation::{LineCap, TessellationResult};
use lyon_tessellation::path::Path;
use ptya_common::ui::animation::spectrum::{LerpSpectrum, Spectrum};

// Circle
pub struct CircleStyle {
    pub radius: LerpSpectrum<f32>,
    pub fill: LerpSpectrum<Color32>,
    pub stroke: LerpSpectrum<Stroke>,
}

impl CircleStyle {
    pub fn draw(&self, x: f32, y: f32, zoom: f32, tess: &mut Tessellator) -> TessellationResult {
        tess.circle(
            Pos2::new(x, y),
            self.radius.get(zoom),
            self.fill.get(zoom),
            self.stroke.get(zoom),
        )
    }
}

// Fill
pub struct PolygonStyle {
    pub fill: LerpSpectrum<Color32>,
    pub stroke: LerpSpectrum<Stroke>,
}

impl PolygonStyle {
	pub fn draw(&self, path: &Vec<Path>, zoom: f32, tess: &mut Tessellator) -> TessellationResult {
		tess.polygon(
			path,
			self.fill.get(zoom),
			self.stroke.get(zoom),
		)
	}
}

// Lines
pub struct LineStyle {
    pub stroke: LerpSpectrum<Stroke>,
    pub cap: Spectrum<LineCap>,
    pub join: Spectrum<LineJoin>,
}

impl LineStyle {
	pub fn draw(&self, path: &Path, zoom: f32, tess: &mut Tessellator) -> TessellationResult {
		let cap = self.cap.get(zoom);
		tess.s_opt.end_cap = cap;
		tess.s_opt.start_cap = cap;
		tess.s_opt.line_join = match self.join.get(zoom) {
			LineJoin::Miter(limit) => {
				tess.s_opt.miter_limit = limit / tess.scale;
				lyon_tessellation::LineJoin::Miter
			}
			LineJoin::Round => {
				lyon_tessellation::LineJoin::Round
			}
			LineJoin::Bevel => {
				lyon_tessellation::LineJoin::Bevel
			}
		};
		tess.path(
			path,
			Color32::TRANSPARENT,
			self.stroke.get(zoom),
		)?;

		// reset values
		tess.s_opt.end_cap = LineCap::Butt;
		tess.s_opt.start_cap = LineCap::Butt;
		tess.s_opt.line_join = lyon_tessellation::LineJoin::Bevel;
		Ok(())
	}
}

#[derive(Clone)]
pub enum LineJoin {
    // f32: limit
    Miter(f32),
    Round,
    Bevel,
}
