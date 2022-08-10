use lyon_tessellation::math::Point;
use lyon_tessellation::path::{Path, PathEvent};
use protobuf::Message;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::Instant;
use fxhash::FxHashMap;

mod parsing {
    include!(concat!(env!("OUT_DIR"), "/protobuf/mod.rs"));
}

const UNIT_SCALE: f32 = 4096.0;

pub enum Geometry {
    Point { x: f32, y: f32 },
    Line { path: Path },
    Polygon { polygons: Vec<Path> },
}

#[derive(Clone)]
pub struct LineLoop {
    pub x: f32,
    pub y: f32,
    pub values: Vec<(f32, f32)>,
}

impl Geometry {
    pub fn new(ty: GeomType, commands: &[Command]) -> Option<Geometry> {
        match ty {
            GeomType::Point => {
                if let Some(Command::MoveTo(values)) = commands.first() {
                    if let Some((x, y)) = values.first() {
                        return Some(Geometry::Point {
                            x: *x as f32 / UNIT_SCALE,
                            y: *y as f32 / UNIT_SCALE,
                        });
                    }
                }
                None
            }
            GeomType::LineString => {
                let mut deque = VecDeque::from_iter(commands);
                let mut builder = Path::builder();

                let mut x = 0.0;
                let mut y = 0.0;
                while !deque.is_empty() {
                    if let Some(Command::MoveTo(values)) = deque.pop_front() {
                        let (dx, dy) = values[0];
                        x += dx as f32 / UNIT_SCALE;
                        y += dy as f32 / UNIT_SCALE;

                        builder.begin(Point::new(x, y));

                        if let Some(Command::LineTo(values)) = deque.pop_front() {
                            for (dx, dy) in values {
                                x += *dx as f32 / UNIT_SCALE;
                                y += *dy as f32 / UNIT_SCALE;
                                builder.line_to(Point::new(x, y));
                            }
                        }

                        builder.end(false);
                    }
                }

                Some(Geometry::Line {
                    path: builder.build(),
                })
            }
            GeomType::Polygon => {
                let mut deque = VecDeque::from_iter(commands);
                let mut x = 0i32;
                let mut y = 0i32;

                let mut chains: Vec<(Vec<(i32, i32)>, Vec<Vec<(i32, i32)>>)> = Vec::new();
                while !deque.is_empty() {
                    if let Some(Command::MoveTo(values)) = deque.pop_front() {
                        let (dx, dy) = values[0];
                        x += dx;
                        y += dy;

                        let mut points = Vec::new();
                        let start = (x, y);
                        points.push(start);
                        if let Some(Command::LineTo(values)) = deque.pop_front() {
                            for (dx, dy) in values {
                                x += *dx;
                                y += *dy;
                                points.push((x, y));
                            }
                        }

                        if let Some(Command::ClosePath) = deque.pop_front() {
                            points.push(start);
                            if shoelace(&points) > 0.0 {
                                // External
                                chains.push((points, Vec::new()));
                            } else {
                                let (_, internal) = chains.last_mut().unwrap();
                                internal.push(points);
                            }
                        }
                    }
                }

                fn points_to_path(points: &[(i32, i32)], internal: &Vec<Vec<(i32, i32)>>) -> Path {
                    let mut iter = points.iter();
                    let mut builder = Path::builder();


                    let (x, y) = iter.next().unwrap();
                    builder.begin(Point::new(
                        (*x as f32) / UNIT_SCALE,
                        (*y as f32) / UNIT_SCALE
                    ));
                    for (x, y) in iter {
                        builder.line_to(Point::new(
                            (*x as f32) / UNIT_SCALE,
                            (*y as f32) / UNIT_SCALE
                        ));
                    }
                    builder.end(false);

                    for iter in internal {
                        let mut iter = iter.iter();
                        let (x, y) = iter.next().unwrap();
                        builder.begin(Point::new(
                            (*x as f32) / UNIT_SCALE,
                            (*y as f32) / UNIT_SCALE
                        ));
                        for (x, y) in iter {
                            builder.line_to(Point::new(
                                (*x as f32) / UNIT_SCALE,
                                (*y as f32) / UNIT_SCALE
                            ));
                        }
                        builder.end(false);
                    }

                    builder.build()
                }

                let mut polygons = Vec::new();
                for (external, internal) in chains {
                    polygons.push(points_to_path(&external, &internal));
                }

                Some(Geometry::Polygon { polygons })
            }
            GeomType::Unknown => {
                panic!("unknwon")
            }
            _ => None,
        }
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum GeomType {
    Unknown,
    Point,
    LineString,
    Polygon,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    F32(f32),
    F64(f64),
    I64(i64),
    U64(u64),
    Bool(bool),
}

pub struct Feature {
    pub id: Option<u64>,
    pub fields: FxHashMap<String, Value>,
    pub geometry_type: GeomType,
    pub geo: Geometry,
}

#[derive(Debug)]
pub enum Command {
    MoveTo(Vec<(i32, i32)>),
    LineTo(Vec<(i32, i32)>),
    ClosePath,
}

pub struct Layer {
    pub version: u32,
    pub name: String,
    pub features: Vec<Feature>,
}

pub struct Tile {
    pub layers: Vec<Layer>,
    pub last_seen: Instant,
}

impl Command {
    fn from_proto(mut commands: VecDeque<u32>) -> Vec<Command> {
        let mut out = Vec::new();
        while let Some(command) = commands.pop_front() {
            let id = command & 0x7;
            let count = command >> 3;
            out.push(match id {
                1 | 2 => {
                    let mut sub_commands = Vec::new();
                    for _ in 0..count {
                        let x = commands.pop_front().unwrap() as i32;
                        let y = commands.pop_front().unwrap() as i32;
                        sub_commands.push((((x >> 1) ^ (-(x & 1))), ((y >> 1) ^ (-(y & 1)))));
                    }

                    if id == 1 {
                        Command::MoveTo(sub_commands)
                    } else {
                        Command::LineTo(sub_commands)
                    }
                }
                7 => Command::ClosePath,
                v => panic!("Unknown id {v}"),
            });
        }

        out
    }
}

impl GeomType {
    fn from_proto(ty: parsing::mvt::tile::GeomType) -> GeomType {
        match ty {
            parsing::mvt::tile::GeomType::UNKNOWN => GeomType::Unknown,
            parsing::mvt::tile::GeomType::POINT => GeomType::Point,
            parsing::mvt::tile::GeomType::LINESTRING => GeomType::LineString,
            parsing::mvt::tile::GeomType::POLYGON => GeomType::Polygon,
        }
    }
}

impl Value {
    fn from_proto(value: parsing::mvt::tile::Value) -> Value {
        if let Some(v) = value.string_value {
            return Value::String(v);
        }
        if let Some(v) = value.float_value {
            return Value::F32(v);
        }
        if let Some(v) = value.double_value {
            return Value::F64(v);
        }
        if let Some(v) = value.int_value {
            return Value::I64(v);
        }
        if let Some(v) = value.uint_value {
            return Value::U64(v);
        }
        if let Some(v) = value.sint_value {
            return Value::I64(v);
        }
        if let Some(v) = value.bool_value {
            return Value::Bool(v);
        }
        panic!("tf");
    }
}

impl Feature {
    fn from_proto(layer: parsing::mvt::tile::Feature, keys: &[String], values: &[Value]) -> Option<Feature> {
        let ty = GeomType::from_proto(layer.type_.unwrap().unwrap());
        let vec = Command::from_proto(VecDeque::from(layer.geometry));
        let fields: Vec<u32> = layer.tags;
        Some(Feature {
            id: layer.id,
            fields: fields.chunks(2).map(|v| {
                let key = keys[v[0] as usize].clone();
                let value = values[v[1] as usize].clone();
                (key, value)
            }).collect(),
            geometry_type: ty,
            geo: Geometry::new(ty, &vec)?,
        })
    }
}

impl Layer {
    fn from_proto(layer: parsing::mvt::tile::Layer) -> Layer {
        let keys: Vec<String> = layer.keys;
        let values: Vec<Value> = layer
            .values
            .into_iter()
            .map(Value::from_proto)
            .collect();
        Layer {
            version: layer.version.unwrap_or(1),
            name: layer.name.unwrap(),
            features: layer
                .features
                .into_iter()
                .flat_map(|v| Feature::from_proto(v, &keys, &values))
                .collect(),
        }
    }
}

impl Tile {
    pub fn from_bytes(bytes: &[u8]) -> anyways::Result<Tile, protobuf::Error> {
        let tile = parsing::mvt::Tile::parse_from_bytes(bytes)?;
        Ok(Tile {
            layers: tile.layers.into_iter().map(Layer::from_proto).collect(),
            last_seen: Instant::now()
        })
    }
}

fn shoelace(v: &Vec<(i32, i32)>) -> f32 {
    let n = v.len();

    let mut area = 0.0;
    let mut j = n - 1;
    for i in 0..n {
        let (from_x, from_y) = v[j];
        let (to_x, to_y) = v[i];
        area += (from_x as f32 * to_y as f32) - (to_x as f32 * from_y as f32);
        j = i;
    }

    (area / 2.0)
}
