pub mod interpolation;
pub mod lerp;
pub mod spectrum;
pub mod state;
pub mod transition;

use crate::settings::ANIMATION_TIME;
use crate::ui::animation::lerp::Lerp;
use egui::{Context, Id, Ui};
use fxhash::FxHashMap;
use std::collections::hash_map::Entry;
use tokio::time::Instant;

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

pub struct AnimationComp {
    animations: FxHashMap<Id, Animation>,
    pub time: f64,
}

impl AnimationComp {
	pub fn new() -> AnimationComp {
		AnimationComp {
			animations: Default::default(),
			time: 0.0
		}
	}

    pub fn val(&mut self, id: Id) -> f32 {
        let time = self.time;
        self.get(id).get_value(time)
    }

    pub fn set(&mut self, id: Id, animation: Animation) -> Option<Animation> {
        self.animations.insert(id, animation)
    }

    pub fn get(&mut self, id: Id) -> &mut Animation {
        self.animations.entry(id).or_insert_with(Animation::empty)
    }

	pub fn tick(&mut self, ctx: &Context) {
		for animation in self.animations.values() {
			if animation.is_active(self.time) {
				ctx.request_repaint();
				break;
			}
		}
		self.time = ctx.input().time;
	}
}

pub struct Animation {
    pub from: f32,
    pub to: f32,
    pub easing: Easing,
    // seconds time
    start: f64,
    duration: f64,
}

impl Animation {
    pub fn empty() -> Animation {
        Animation {
            from: 0.0,
            to: 0.0,
            easing: Easing::EaseInOut,
            start: 0.0,
            duration: 0.0,
        }
    }

    pub fn get_pos(&self, time: f64) -> f64 {
        if self.duration == 0.0 {
            1.0
        } else {
            (time - self.start) / self.duration
        }
    }

    /// Overwrites the current source value
    pub fn set_from(&mut self, from: f32) -> &mut Self {
        self.from = from;
        self
    }

    /// Overwrites the current target value
    pub fn set_to(&mut self, to: f32) -> &mut Self {
        self.to = to;
        self
    }

    /// Overwrites the current easing
    pub fn set_easing(&mut self, easing: Easing) -> &mut Self {
        self.easing = easing;
        self
    }

	/// Removes any current animation and sets a static value.
    pub fn set_current(&mut self, value: f32) -> &mut Self {
        self.start = 0.0;
        self.duration = 0.0;
        self.from = value;
        self.to = value;
        self
    }

    /// Starts a new animation to a new target.
    pub fn start(&mut self, time: f64, to: f32, speed: f32) {
        let current_value = self.get_value(time);
        self.from = current_value;
        self.to = to;
        self.start = time;
        self.duration = (speed * ANIMATION_TIME) as f64;
    }

    /// Checks if the animation is currently moving
    pub fn is_active(&self, time: f64) -> bool {
        let pos = self.get_pos(time);
        pos > 0.0 && pos < 1.0
    }

    /// Gets the current value of the animation
    pub fn get_value(&self, time: f64) -> f32 {
        let time_t = if self.duration == 0.0 {
            1.0
        } else {
            (time - self.start) / self.duration
        };
        let clamped_t = (time_t).clamp(0.0, 1.0);
        let eased_t = self.easing.apply(clamped_t);
        self.from.lerp(&self.to, eased_t as f32)
    }
}

pub enum Easing {
    Linear,
    // Quart (its a touch more aggressive than cubic)
    EaseIn,
    EaseOut,
    EaseInOut,
}

impl Easing {
    #[inline(always)]
    pub fn apply(&self, x: f64) -> f64 {
        assert!((0.0..=1.0).contains(&x));
        match self {
            Easing::Linear => x,
            Easing::EaseIn => x * x * x * x,
            Easing::EaseOut => 1.0 - (1.0 - x).powf(4.0),
            Easing::EaseInOut => {
                if x < 0.5 {
                    8.0 * x * x * x * x
                } else {
                    1.0 - (-2.0 * x + 2.0).powf(4.0) / 2.0
                }
            }
        }
    }
}
