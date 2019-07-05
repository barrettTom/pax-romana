use ggez::filesystem::File;
use ggez::graphics::Rect;
use std::collections::HashMap;
use std::f32::consts::PI;

use crate::animations::Animation;
use crate::constants::{self, FLIP_A, FLIP_D, FLIP_H, FLIP_V};
use crate::property::Property;
use crate::xmlelements::XMLElements;

#[derive(Clone, Debug, PartialEq)]
pub struct Tile {
    pub source: Rect,
    pub property: Property,
}

impl Tile {
    pub fn new(source: Rect, property: Property) -> Tile {
        Tile { source, property }
    }

    pub fn flip(&mut self) {
        self.source.x *= -1.0;
        self.source.x -= self.source.w;
    }
}

fn flip(tile: Tile) -> Tile {
    let mut t = tile.clone();
    t.source.x *= -1.0;
    t.source.x -= t.source.w;
    t
}

impl Default for Tile {
    fn default() -> Tile {
        Tile::new(Rect::zero(), Property::default())
    }
}

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

                let property = match tile_element {
                    Some(tile_element) => {
                        Property::new(elements.get_children(&tile_element, "property"))
                    }
                    None => Property::default(),
                };

                tiles.insert(id, Tile::new(Rect::new(x, y, w, h), property));
                id += 1;
            }
        }

        for (id, tile) in tiles.clone().into_iter() {
            for i in 1..8 {
                let (new_id, new_tile) = match i {
                    1 => ((id | FLIP_H), tile.clone()),
                    2 => ((id | FLIP_V), flip(tile.clone())),
                    3 => ((id | FLIP_D), tile.clone()),
                    4 => ((id | FLIP_D | FLIP_H), tile.clone()),
                    5 => ((id | FLIP_D | FLIP_V), tile.clone()),
                    6 => ((id | FLIP_H | FLIP_V), tile.clone()),
                    7 => ((id | FLIP_A), tile.clone()),
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
            .filter(|(_, t)| t.property.spawn.is_some())
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

        if first_tile.property.entity.is_some() {
            Animation::new(
                self.tiles
                    .values()
                    .cloned()
                    .filter(|t| t.property.entity == first_tile.property.entity)
                    .collect(),
            )
        } else {
            Animation::new(vec![first_tile])
        }
    }

    pub fn get_tile_by_entity_keyframe(&self, entity: &str, keyframe: usize) -> Tile {
        let tile_id = self
            .tiles
            .iter()
            .find(|(_, t)| {
                t.property.entity == Some(entity.to_string())
                    && Some(keyframe) == t.property.keyframe
            })
            .unwrap()
            .0;

        self.tiles.get(tile_id).unwrap().clone()
    }
}

pub fn convert_angle_to_rad(angle: f32) -> f32 {
    angle * (PI / 180.0)
}
