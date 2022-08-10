use crate::ui::animation::lerp::Lerp;
use crate::ui::animation::state::State;
use crate::ui::animation::ContextHolder;
use egui::{Id, Ui};

/// A constant value that changes with time.
pub struct Transition<L: Lerp> {
    pub state: State,
    pub from: L,
    pub to: L,
}

impl<V: Lerp> Transition<V> {
    pub fn state(&mut self) -> &mut State {
        &mut self.state
    }

    pub fn set(&mut self, ctx: &impl ContextHolder, to: V) {
        if !self.state.is_nan() && to != self.to {
            let value = self.get(ctx);
            self.from = value;
            self.to = to;
            self.state.reset_target(ctx, 0.0, 1.0);
        }
    }

    pub fn reset(&mut self, ctx: &impl ContextHolder, value: V) {
        if !self.state.is_nan() {
            self.from = value.clone();
            self.to = value;
            self.state.reset_target(ctx, 0.0, 1.0);
        }
    }

    pub fn get(&mut self, ctx: &impl ContextHolder) -> V {
        if self.state.is_nan() {
            self.from.clone()
        } else if self.state.is_done() {
            self.to.clone()
        } else {
            V::lerp(
                &self.from,
                &self.to,
                self.state.get_progress(ctx),
            )
        }
    }
}