use palette::{Lab, Srgb};
use palette::convert::FromColorUnclamped;
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
        let v0_l = lab(*v0);
        let v1_l = lab(*v1);
        from_lab(Lab {
            l: (((v1_l.l - v0_l.l) * t) + v0_l.l),
            a: (((v1_l.a - v0_l.a) * t) + v0_l.a),
            b: (((v1_l.b - v0_l.b) * t) + v0_l.b),
            white_point: Default::default(),
        },  (((v1.a() as f32 - v0.a() as f32) * t) + v0.a() as f32) as u8)
    }
}

fn lab(color: Color32) -> Lab {
    let rgb = Srgb::new(color.r(), color.g(), color.b());
    let rgb: Srgb<f32> = rgb.into_format();
    Lab::from_color_unclamped(rgb)
}

fn from_lab(lab: Lab, a: u8) -> Color32 {
    let rgb2 = Srgb::from_color_unclamped(lab);
    let rgb1: Srgb<u8> = rgb2.into_format();
    Color32::from_rgba_premultiplied(rgb1.red, rgb1.green, rgb1.blue, a)
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
