use crate::geometry::path::{MultiPathGeometry, PathGeometry};
use crate::style::Style;
use crate::types::MapVertex;
use mathie::Vec2D;

#[derive(Copy, Clone)]
pub struct StrokeStyle {
    pub color: [f32; 4],
    pub width: f32,
}

impl StrokeStyle {
    pub fn new(color: [u8; 3], width: f32) -> StrokeStyle {
        StrokeStyle {
            color: [
                color[0] as f32 / 255.0,
                color[1] as f32 / 255.0,
                color[2] as f32 / 255.0,
                1.0,
            ],
            width,
        }
    }
}

#[derive(Copy, Clone)]
pub struct StrokeStyleInput<'a> {
    pub path: &'a [Vec2D<f32>],
}

impl<'a> From<&'a PathGeometry> for StrokeStyleInput<'a> {
    fn from(path: &'a PathGeometry) -> Self {
        StrokeStyleInput { path: &path.points }
    }
}

impl<'a> From<&'a [Vec2D<f32>]> for StrokeStyleInput<'a> {
    fn from(path: &'a [Vec2D<f32>]) -> Self {
        StrokeStyleInput { path }
    }
}

impl Style for StrokeStyle {
    type Input<'a> = StrokeStyleInput<'a>;

    fn get_len(input: Self::Input<'_>) -> usize {
    	input.path.len() * 2
    }

    fn compile(&self, input: Self::Input<'_>, v: &mut Vec<MapVertex>, i: &mut Vec<u32>) {
        let half_width = (1.0 /  8192.0) / 2.0;

        let start = v.len();
        let len = input.path.len() * 2;
        for _ in 0..len {
            v.push(MapVertex {
                a_pos: [0.0, 0.0],
                a_color: self.color,
            });
        }

        let mut pos = 0;
        PathCursor {
            points: input.path,
            pos: 0,
        }
        .visit_points(|value, side| {
            let vertex = value + (side * half_width);
            v[start + pos].a_pos = vertex.into();

            let vertex = value + ((-side) * half_width);
            v[start + ((len - 1) - pos)].a_pos = vertex.into();
            pos += 1;
        });

        let mut data = Vec::with_capacity(len * 2);
        for vertex in &mut v[start..(start + len)] {
            data.push(vertex.a_pos[0] * 8192.0);
            data.push(vertex.a_pos[1] * 8192.0);
        }

        for idx in earcutr::earcut(&data, &Vec::new(), 2) {
            i.push((idx + start) as u32);
        }
    }

    fn needs_update(&self, old_styler: Self) -> bool {
        self.color != old_styler.color || self.width != old_styler.width
    }

    fn update(&self, input: Self::Input<'_>, v: &mut [MapVertex], old_styler: Option<Self>) {
        // we do very cheap shit when only the color changes
        let mut only_color = false;
        if let Some(styler) = old_styler {
            only_color = styler.width == self.width;
        }

        if only_color {
            // very cheap shit as you can see
            for vertex in v {
                vertex.a_color = self.color;
            }
        } else {
            // not as cheap as you can see.
            let half_width = self.width / 2.0;
            let len = v.len();
            let mut pos = 0;
            PathCursor {
                points: input.path,
                pos: 0,
            }
            .visit_points(|value, side| {
                let vertex = value + (side * half_width);
                v[pos] = MapVertex {
                    a_pos: vertex.into(),
                    a_color: self.color,
                };

                let vertex = value + ((-side) * half_width);
                v[(len - 1) - pos] = MapVertex {
                    a_pos: vertex.into(),
                    a_color: self.color,
                };
                pos += 1;
            });
        }
    }

    fn prepare(&mut self, scale: f32) {
        self.width *= scale;
    }
}

pub struct PathCursor<'a> {
    points: &'a [Vec2D<f32>],
    pos: usize,
}

impl<'a> PathCursor<'a> {
    pub fn peek_next(&mut self) -> Option<Vec2D<f32>> {
        if self.pos != self.points.len() {
            Some(self.points[self.pos])
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Option<Vec2D<f32>> {
        if self.pos != self.points.len() {
            let point = self.points[self.pos];
            self.pos += 1;
            Some(point)
        } else {
            None
        }
    }

    pub fn peek_prev(&mut self) -> Option<Vec2D<f32>> {
        if self.pos != 1 {
            Some(self.points[self.pos - 2])
        } else {
            None
        }
    }

    pub fn visit_points(&mut self, mut func: impl FnMut(Vec2D<f32>, Vec2D<f32>)) {
        fn get_side(start: Vec2D<f32>, stop: Vec2D<f32>) -> Vec2D<f32> {
            let furbertensvector = stop - start;
            Vec2D::new(-furbertensvector.y(), furbertensvector.x()).normalize()
        }

        while let Some(value) = self.next() {
            let prev = self.peek_prev().map(|prev| get_side(prev, value));
            let next = self.peek_next().map(|next| get_side(value, next));

            let side = match (prev, next) {
                (Some(v0), Some(v1)) => v0.lerp(v1, 0.5).normalize(),
                (Some(v0), None) => v0,
                (None, Some(v0)) => v0,
                _ => {
                    panic!("we fucked up");
                }
            };

            func(value, side);
        }
    }
}
