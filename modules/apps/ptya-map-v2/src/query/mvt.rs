use std::collections::VecDeque;
use std::time::Instant;

use atlas::geometry::path::{MultiPathGeometry, PathGeometry};
use fxhash::FxHashMap;
use mathie::Vec2D;
use protobuf::Message;
use atlas::geometry::fill::{MultiPolygonGeometry, PolygonGeometry};
use atlas::geometry::GeometryData;
use atlas::data::{FeatureData, LayerData, TileData};

mod parsing {
    include!(concat!(env!("OUT_DIR"), "/protobuf/mod.rs"));
}

const UNIT_SCALE: f32 = 4096.0;

enum Geometry {
    Point { pos: Vec<Vec2D<f32>> },
    Line(MultiPathGeometry),
    Polygon(MultiPolygonGeometry),
}

struct Point {
    pub pos: Vec2D<f32>,
}

struct Polygon {
    pub outer: Vec<Vec2D<f32>>,
    pub inner: Vec<Vec<Vec2D<f32>>>,
}

impl Geometry {
    pub fn new(ty: GeomType, commands: &[Command]) -> Option<Geometry> {
        match ty {
            GeomType::Point => {
                let mut deque = VecDeque::from_iter(commands);

                let mut points = Vec::new();
                while let Some(Command::MoveTo(values)) = deque.pop_front() {
                    if let Some((x, y)) = values.first() {
                        points.push(Vec2D::new(*x as f32 / UNIT_SCALE, *y as f32 / UNIT_SCALE));
                    }
                }

                Some(Geometry::Point { pos: points })
            }
            GeomType::LineString => {
                let mut deque = VecDeque::from_iter(commands);

                let mut paths = Vec::new();
                let mut x = 0.0;
                let mut y = 0.0;
                while let Some(Command::MoveTo(values)) = deque.pop_front() {
                    let mut points = Vec::new();

                    let (dx, dy) = values[0];
                    x += (dx as f32 / UNIT_SCALE);
                    y += (dy as f32 / UNIT_SCALE);
                    points.push(Vec2D::new(x, y));

                    if let Some(Command::LineTo(values)) = deque.pop_front() {
                        for (dx, dy) in values {
                            x += *dx as f32 / UNIT_SCALE;
                            y += (*dy as f32 / UNIT_SCALE);
                            points.push(Vec2D::new(x, y));
                        }
                    }

                    paths.push(PathGeometry {
                        points
                    });
                }

                Some(Geometry::Line(MultiPathGeometry { paths }))
            }
            GeomType::Polygon => {
                let mut deque = VecDeque::from_iter(commands);
                let mut x = 0f32;
                let mut y = 0f32;
                
                let mut polygons = Vec::new();    
                let mut current_polygon = PolygonGeometry {
                    outer: vec![],
                    inner: vec![],
                };
                while let Some(Command::MoveTo(values)) = deque.pop_front() {
                    let (dx, dy) = values[0];
                    x += dx as f32 / UNIT_SCALE;
                    y += dy as f32 / UNIT_SCALE;

                    let mut points = Vec::new();
                    let start =Vec2D::new(x, y);
                    points.push(start);

                    if let Some(Command::LineTo(values)) = deque.pop_front() {
                        for (dx, dy) in values {
                            x += *dx as f32 / UNIT_SCALE;
                            y += *dy as f32 / UNIT_SCALE;
                            points.push(Vec2D::new(x, y));
                        }
                    }

                    if let Some(Command::ClosePath) = deque.pop_front() {
                        points.push(start);
                        if shoelace(&points) > 0.0 {
                            polygons.push(current_polygon);
                            current_polygon = PolygonGeometry {
                                outer: points,
                                inner: vec![],
                            };
                        } else {
                            current_polygon.inner.push(points);
                        }
                    }
                }
                
                if !current_polygon.outer.is_empty() {
                    polygons.push(current_polygon);
                }

                Some(Geometry::Polygon(MultiPolygonGeometry {
                    polygons
                }))
            }
            GeomType::Unknown => {
                panic!("unknwon")
            }
            _ => None,
        }
    }
    
    pub fn build(self) -> Option<GeometryData> {
        match self {
            Geometry::Point { .. } => None,
            Geometry::Line(path) => {
                Some(GeometryData::Path(path))
            }
            Geometry::Polygon(polygon) => Some(GeometryData::Fill(polygon)),
        }
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
enum GeomType {
    Unknown,
    Point,
    LineString,
    Polygon,
}

#[derive(Debug, Clone, PartialEq)]
enum Value {
    String(String),
    F32(f32),
    F64(f64),
    I64(i64),
    U64(u64),
    Bool(bool),
}

struct Feature {
    pub id: Option<u64>,
    pub fields: FxHashMap<String, Value>,
    pub geometry_type: GeomType,
    pub geo: Geometry,
}

#[derive(Debug)]
enum Command {
    MoveTo(Vec<(i32, i32)>),
    LineTo(Vec<(i32, i32)>),
    ClosePath,
}

struct Layer {
    pub name: String,
    pub features: Vec<Feature>,
}

struct Tile {
    pub layers: Vec<Layer>,
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
    
    fn build(self) -> atlas::data::Value {
        match self {
            Value::String(v) => atlas::data::Value::String(v),
            Value::F32(v) => atlas::data::Value::F32(v),
            Value::F64(v) => atlas::data::Value::F64(v),
            Value::I64(v) => atlas::data::Value::I64(v),
            Value::U64(v) => atlas::data::Value::U64(v),
            Value::Bool(v) => atlas::data::Value::Bool(v),
        }
    }
    
    
}

impl Feature {
    fn from_proto(
        layer: parsing::mvt::tile::Feature,
        keys: &[String],
        values: &[Value],
    ) -> Option<Feature> {
        let ty = GeomType::from_proto(layer.type_.unwrap().unwrap());
        let vec = Command::from_proto(VecDeque::from(layer.geometry));
        let fields: Vec<u32> = layer.tags;
        Some(Feature {
            id: layer.id,
            fields: fields
                .chunks(2)
                .map(|v| {
                    let key = keys[v[0] as usize].clone();
                    let value = values[v[1] as usize].clone();
                    (key, value)
                })
                .collect(),
            geometry_type: ty,
            geo: Geometry::new(ty, &vec)?,
        })
    }

    fn build(self) -> Option<FeatureData> {
        Some(FeatureData {
            fields: self.fields.into_iter().map(|(v, k)| {
                (v, k.build())
            }).collect(),
            geometry: self.geo.build()?
        })
    }
}

impl Layer {
    fn from_proto(layer: parsing::mvt::tile::Layer) -> Layer {
        let keys: Vec<String> = layer.keys;
        let values: Vec<Value> = layer.values.into_iter().map(Value::from_proto).collect();
        Layer {
            name: layer.name.unwrap(),
            features: layer
                .features
                .into_iter()
                .flat_map(|v| Feature::from_proto(v, &keys, &values))
                .collect(),
        }
    }

    fn build(self) -> LayerData {
        LayerData {
            name: self.name,
            features: self.features.into_iter().flat_map(|v| v.build()).collect()
        }
    }
}

impl Tile {
    fn from_bytes(bytes: &[u8]) -> anyways::Result<Tile, protobuf::Error> {
        let tile = parsing::mvt::Tile::parse_from_bytes(bytes)?;
        Ok(Tile {
            layers: tile.layers.into_iter().map(Layer::from_proto).collect(),
        })
    }
    
    fn build(self) -> TileData {
        TileData {
            layers: self.layers.into_iter().map(|v| v.build()).collect()
        }
    }
}

pub fn parse_mvt(bytes: &[u8]) -> anyways::Result<TileData, protobuf::Error> {
    let tile = Tile::from_bytes(bytes)?;
    Ok(tile.build())
}

fn shoelace(v: &Vec<Vec2D<f32>>) -> f32 {
    let n = v.len();

    let mut area = 0.0;
    let mut j = n - 1;
    for i in 0..n {
        let from = v[j];
        let to = v[i];
        area += (from.x() * to.y() ) - (to.x() * from.y());
        j = i;
    }

    (area / 2.0)
}
