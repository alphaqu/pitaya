use egui::{Color32, Rect, Stroke, Vec2};

pub trait Lerp: PartialEq + Clone {
    fn lerp(v0: &Self, v1: &Self, t: f32) -> Self;
}

impl Lerp for Vec2 {
    fn lerp(v0: &Self, v1: &Self, t: f32) -> Self {
        ((*v1 - *v0) * t) + *v0
    }
}

impl Lerp for f32 {
    fn lerp(v0: &Self, v1: &Self, t: f32) -> Self {
        ((v1 - v0) * t) + v0
    }
}

impl Lerp for Color32 {
    fn lerp(v0: &Self, v1: &Self, t: f32) -> Self {
        Color32::from_rgba_premultiplied(
            (((v1.r() as f32 - v0.r() as f32) * t) + v0.r() as f32) as u8,
            (((v1.g() as f32 - v0.g() as f32) * t) + v0.g() as f32) as u8,
            (((v1.b() as f32 - v0.b() as f32) * t) + v0.b() as f32) as u8,
            (((v1.a() as f32 - v0.a() as f32) * t) + v0.a() as f32) as u8,
        )
    }
}

impl Lerp for Stroke {
    fn lerp(v0: &Self, v1: &Self, t: f32) -> Self {
        Stroke {
            width: Lerp::lerp(&v0.width, &v1.width, t),
            color: Lerp::lerp(&v0.color, &v1.color, t)
        }
    }
}

impl Lerp for Rect {
    fn lerp(v0: &Self, v1: &Self, t: f32) -> Self {
        let size = Vec2::lerp(&v0.size(), &v1.size(), t);
        let center = Vec2::lerp(&v0.center().to_vec2(), &v1.center().to_vec2(), t);
        Rect::from_center_size(center.to_pos2(), size)
    }
}
