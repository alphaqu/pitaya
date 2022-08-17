//! Custom Piyaya widgets.
use crate::settings::SPACING_SIZE;
use crate::{ColorState, ColorType, System, INTERACTIVE_SIZE};
use egui::{Rect, Response, Sense, Ui, Vec2};
use epaint::{Color32, Rounding, Stroke};

pub mod button;
pub mod gradient;
pub mod progress_spinner;
pub mod slider;
pub mod text;

pub fn alloc_intractable(ui: &mut Ui, content_width: f32) -> (Rect, Response, Rounding) {
    let (rect, response) = ui.allocate_at_least(
        Vec2::new(
            SPACING_SIZE + content_width + SPACING_SIZE,
            INTERACTIVE_SIZE,
        ),
        Sense::click_and_drag(),
    );

    (rect, response, Rounding::same(INTERACTIVE_SIZE / 2.0))
}
