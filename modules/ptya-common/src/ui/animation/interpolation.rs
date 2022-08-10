use crate::ui::animation::lerp::Lerp;

pub enum Interpolation {
	// Basic
	/// No interpolation
	Step,
	/// Linear interpolation
	Linear,
	/// Exponential interpolation
	Exponential(f32),
}

impl Interpolation {
	pub fn get(&self, t: f32) -> f32 {
		match self {
			Interpolation::Step => 0.0,
			Interpolation::Linear => t,
			Interpolation::Exponential(base) => {
				t.powf(*base)
			}
		}
	}
}