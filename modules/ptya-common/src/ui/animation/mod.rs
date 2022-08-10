pub mod state;
pub mod transition;
pub mod lerp;
pub mod interpolation;
pub mod spectrum;

use egui::{Context, Ui};

pub trait ContextHolder {
	fn get_context(&self) -> &Context;
}

impl ContextHolder for Ui {
	fn get_context(&self) -> &Context {
		self.ctx()
	}
}

impl ContextHolder for Context {
	fn get_context(&self) -> &Context {
		self
	}
}
