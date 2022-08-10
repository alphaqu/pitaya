use mathie::Vec2D;
use crate::types::MapVertex;
use delaunator::Point;

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
}

pub fn get_len_path(points: &[Vec2D<f32>]) -> usize {
    points.len() * 2
}

pub fn tessellate_path(
    points: &[Vec2D<f32>],
    width: f32,
    color: [f32; 4],
    output: &mut [MapVertex]
) {
    let mut cursor = PathCursor { points, pos: 0 };
    let half_width = width / 2.0;
    
    if output.len() != get_len_path(points) {
        panic!("invalid length");
    }
    
    let len = output.len();
    let mut pos = 0;
    while let Some(value) = cursor.next() {
        let prev = cursor.peek_prev().map(|prev| get_side(prev, value));
        let next = cursor.peek_next().map(|next| get_side(value, next));

        let side = match (prev, next) {
            (Some(v0), Some(v1)) => v0.lerp(v1, 0.5),
            (Some(v0), None) => v0,
            (None, Some(v0)) => v0,
            _ => {
                panic!("we fucked up");
            }
        };

        output[pos] = MapVertex {
            a_pos: (value + (side * half_width)).into(),
            a_color: color
        };
        output[(len - 1) - pos] = MapVertex {
            a_pos: (value + ((-side) * half_width)).into(),
            a_color: color
        };
        pos += 1;
    }
}

fn get_side(start: Vec2D<f32>, stop: Vec2D<f32>) -> Vec2D<f32> {
    let furbertensvector = stop - start;
    Vec2D::new(-*furbertensvector.y(), *furbertensvector.x()).normalize()
}

pub fn compile_indices(outer: &[MapVertex], inner: &[&[MapVertex]]) -> Vec<u32> {
    let mut data = Vec::new();
    let mut hole_indices = Vec::new();
    for point in outer {
        data.push(point.a_pos[0]);
        data.push(point.a_pos[1]);
    }

    for ring in inner {
        hole_indices.push(data.len() / 2);
        for point in *ring {
            data.push(point.a_pos[0]);
            data.push(point.a_pos[1]);
        }
    }
    
    let indices = earcutr::earcut(&data, &hole_indices, 2);
    indices.into_iter().map(|v| v as u32).collect()
}
