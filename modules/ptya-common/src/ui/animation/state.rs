use crate::ui::animation::lerp::Lerp;
use crate::ui::animation::ContextHolder;
use egui::{Id};
use log::{debug, trace};
use crate::settings::ANIMATION_TIME;

#[non_exhaustive]
pub struct State {
    id: Id,
    target: f32,
    pub speed: f32,
    is_done: bool,
}

impl State {
    pub fn basic(id: Id, ctx: &impl ContextHolder) -> State {
        State::new(id, 1.0, 1.0, ctx)
    }

    pub fn new(id: Id, to: f32, speed: f32, ctx: &impl ContextHolder) -> State {
        let mut state = State {
            id,
            target: -1.0,
            speed:speed,
            is_done: false,
        };
        state.set_target(ctx, to);
        state
    }

    pub fn get_progress(&mut self, ctx: &impl ContextHolder) -> f32 {
        if self.is_done {
            self.target
        } else {
            let progress = ctx.get_context().animate_value_with_time(
                self.id,
                self.target,
                ANIMATION_TIME * self.speed,
            );

            let progress = ease_in_out_quint(progress);
            if progress == self.target {
                self.is_done = true;
            }

            progress
        }
    }

    pub fn set_target(&mut self, ctx: &impl ContextHolder, value: f32) {
        let value = value.clamp(0.0, 1.0);
        if self.target != value {
            self.target = value;
            self.is_done = false;
            ctx.get_context().animate_value_with_time(
                self.id,
                self.target,
                ANIMATION_TIME * self.speed,
            );
        }
    }

    pub fn reset_target(&mut self, ctx: &impl ContextHolder, from: f32, to: f32) {
        let from = from.clamp(0.0, 1.0);
        let to = to.clamp(0.0, 1.0);
        self.target = to;
        self.is_done = false;
        ctx.get_context()
            .animate_value_with_time(self.id, from, 0.0);
        ctx.get_context().animate_value_with_time(
            self.id,
            to,
            ANIMATION_TIME * self.speed,
        );
    }

    pub fn lerp<V: Lerp>(&mut self, ctx: &impl ContextHolder, from: &V, to: &V) -> V {
        let progress = self.get_progress(ctx);
        V::lerp_static(from, to, progress)
    }

    pub fn is_nan(&self) -> bool {
        self.id == Id::null()
    }

    pub fn id(&self) -> Id {
        self.id
    }
    pub fn is_done(&self) -> bool {
        self.is_done
    }
    pub fn target(&self) -> f32 {
        self.target
    }
}

fn ease_in_out_quint(x: f32) -> f32 {
    if x < 0.5 {
        4.0 * x * x * x
    } else {
        1.0 - (-2.0 * x + 2.0).powf(3.0) / 2.0
    }
}