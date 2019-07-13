use ggez::filesystem::File;
use ggez::graphics::Rect;
use std::collections::HashMap;
use std::f32::consts::PI;

use crate::animations::Animation;
use crate::constants::{self, FLIP_A, FLIP_D, FLIP_H, FLIP_V};
use crate::tile::{Properties, Tile};
use crate::xmlelements::XMLElements;

pub struct Tileset {
    tiles: HashMap<usize, Tile>,
}

impl Tileset {
    pub fn new(file: File) -> Tileset {
        let elements = XMLElements::new(file);

        let height = elements
            .get_element_attribute("image", "height")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let columns = elements
            .get_element_attribute("tileset", "columns")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let rows = height / (constants::TILE_HEIGHT as usize);

        let mut tiles = HashMap::new();
        tiles.insert(0, Tile::default());

        let w = 1.0 / columns as f32;
        let h = 1.0 / rows as f32;
        let mut id = 1;
        for r in 0..rows {
            for c in 0..columns {
                let x = c as f32 / columns as f32;
                let y = r as f32 / rows as f32;

                let tile_element = elements.get_elements("tile").into_iter().find(|e| {
                    XMLElements::get_attribute(e, "id")
                        .unwrap()
                        .parse::<usize>()
                        .unwrap()
                        + 1
                        == id
                });

                let properties = match tile_element {
                    Some(tile_element) => {
                        Properties::new(elements.get_children(&tile_element, "property"))
                    }
                    None => Properties::default(),
                };

                tiles.insert(id, Tile::new(Rect::new(x, y, w, h), properties));
                id += 1;
            }
        }

        for (id, tile) in tiles.clone().into_iter() {
            for i in 1..8 {
                let (new_id, new_tile) = match i {
                    1 => ((id | FLIP_H), flip(tile.clone())),
                    //2 => ((id | FLIP_V), tile.clone()),
                    //3 => ((id | FLIP_D), tile.clone()),
                    4 => ((id | FLIP_D | FLIP_H), rotate(tile.clone(), 90.0)),
                    5 => ((id | FLIP_D | FLIP_V), rotate(tile.clone(), 270.0)),
                    6 => ((id | FLIP_H | FLIP_V), rotate(tile.clone(), 180.0)),
                    7 => ((id | FLIP_A), rotate(tile.clone(), 90.0)),
                    _ => (0, Tile::default()),
                };

                if new_id != 0 {
                    tiles.insert(new_id, new_tile);
                }
            }
        }

        Tileset { tiles }
    }

    pub fn get_spawn_tiles(&self) -> HashMap<usize, Tile> {
        self.tiles
            .clone()
            .into_iter()
            .filter(|(_, t)| t.properties.spawn.is_some())
            .collect()
    }

    pub fn get_animation(&self, tile_id: usize) -> Animation {
        let first_tile = self
            .tiles
            .iter()
            .find(|(id, _)| id == &&tile_id)
            .unwrap()
            .1
            .clone();

        if first_tile.properties.entity.is_some() {
            Animation::new(
                self.tiles
                    .values()
                    .cloned()
                    .filter(|t| {
                        t.properties.entity == first_tile.properties.entity
                            && (t.properties.rotation - first_tile.properties.rotation)
                                < constants::FLOAT_PRECISION
                            && t.source.x.is_sign_positive()
                                == first_tile.source.x.is_sign_positive()
                            && t.source.y.is_sign_positive()
                                == first_tile.source.y.is_sign_positive()
                    })
                    .collect(),
            )
        } else {
            Animation::new(vec![first_tile])
        }
    }

    pub fn get_tile_by_entity_keyframe(&self, entity: &str, keyframe: usize) -> Tile {
        self.tiles
            .values()
            .find(|t| {
                t.properties.entity == Some(entity.to_string())
                    && Some(keyframe) == t.properties.keyframe
                    && t.properties.rotation == 0.0
                    && t.source.x > 0.0
                    && t.source.y > 0.0
            })
            .unwrap()
            .clone()
    }
}

pub fn convert_angle_to_rad(angle: f32) -> f32 {
    angle * (PI / 180.0)
}

fn flip(tile: Tile) -> Tile {
    let mut t = tile.clone();
    t.source.x *= -1.0;
    t.source.x -= t.source.w;
    t
}

fn rotate(tile: Tile, angle: f32) -> Tile {
    let mut t = tile.clone();
    t.properties.rotation = convert_angle_to_rad(angle);
    t
}
