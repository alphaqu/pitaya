use fxhash::FxHashMap;
use log::{info, warn};
use crate::asset::Location;
use crate::AssetComp;
use anyways::Result;
use egui::{Align2, Painter, Pos2, Vec2};
use epaint::{Color32, FontFamily, FontId, Stroke};

pub struct IconComp {
    lookup: FxHashMap<String, char>,
}

impl IconComp {
    pub fn init() -> IconComp {
        IconComp {
            lookup: Default::default()
        }
    }

    pub async fn new(asset: &AssetComp) -> Result<IconComp> {
        let data = asset.read_file(Location::Assets, "fonts/Icons.ttf").await?;
        let face = ttf_parser::Face::from_slice(&data, 0).unwrap();

        let mut chars = Vec::new();
        if let Some(sub_table) =  face.tables().cmap {
            for (id, sub_table) in sub_table.subtables.into_iter().enumerate() {
                if !sub_table.is_unicode() {
                    warn!("SubTable {id} is not unicode compatible");
                    continue;
                }

                sub_table.codepoints(|id| {
                    match char::from_u32(id) {
                        Some(char) => {
                            chars.push(char);
                        }
                        None => {
                            warn!("Glyph {id} failed to convert to char.");
                        }
                    }
                })

            }
        }

        let mut lookup: FxHashMap<String, char> = FxHashMap::default();
        for char in chars {
          if let Some(glyph_id) = face.glyph_index(char) {
              if let Some(name) = face.glyph_name(glyph_id) {
                  lookup.insert(name.to_string(), char);
              } else  {
                  warn!("Glyph {glyph_id:?} does not have a name");
              }
          } else {
              warn!("Character {char} does not exist in the face even thought it exists in its sub_table.")
          }
        }

        info!("Created cache of {} icons", lookup.len());
        Ok(IconComp { lookup })
    }

    pub fn draw(&self, painter: &Painter, name: &str, pos: Pos2, size: f32, color: Color32) {
        let char = self.get(name);
        painter.text(pos + Vec2::new(0.0, size / 5.0), Align2::CENTER_CENTER, char, FontId::new(
            size * 1.50,
            FontFamily::Name("Icons".into()),
        ), color);
    }

    pub fn get(&self, name: &str) -> char {
        *self.lookup.get(name).expect("Could not find font")
    }
}
