use ggez::filesystem::File;
use ggez::graphics::{spritebatch::SpriteBatch, DrawParam};
use ggez::nalgebra::{Point2, Vector2};
use std::f32::consts::PI;
use xml::reader::XmlEvent::Characters;

use crate::constants;
use crate::tileset::Tileset;
use crate::xmlelements::XMLElements;

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub layers: Vec<Layer>,
}

impl Map {
    pub fn new(file: File) -> Map {
        let elements = XMLElements::new(file);

        let width = elements.get_element_attribute("map", "width").unwrap();
        let height = elements.get_element_attribute("map", "height").unwrap();
        let layers = elements
            .events
            .iter()
            .filter_map(|e| {
                if let Characters(text) = e {
                    Some(Layer::new(text.to_string()))
                } else {
                    None
                }
            })
            .collect();

        Map {
            layers,
            width,
            height,
        }
    }

    pub fn draw(&self, spritebatch: &mut SpriteBatch, tileset: &Tileset) {
        for layer in self.layers.iter() {
            for x in 0..self.width {
                for y in 0..self.height {
                    let tile_id = layer.data[x + (y * self.height)];
                    let (tile_id, rotate) = self.decode(tile_id as u32);

                    let offset = (constants::TILE_WIDTH / 2.0) * constants::TILE_SCALE;

                    let draw_param = DrawParam::default()
                        .src(tileset.tiles[tile_id as usize])
                        .rotation(rotate)
                        .offset(Point2::new(0.5, 0.5))
                        .dest(Point2::new(
                            (constants::TILE_WIDTH * constants::TILE_SCALE * x as f32) + offset,
                            (constants::TILE_HEIGHT * constants::TILE_SCALE * y as f32) + offset,
                        ))
                        .scale(Vector2::new(constants::TILE_SCALE, constants::TILE_SCALE));

                    spritebatch.add(draw_param);
                }
            }
        }
    }

    pub fn get_dimensions(&self) -> (f32, f32) {
        (
            (constants::TILE_WIDTH * constants::TILE_SCALE) * self.width as f32,
            (constants::TILE_HEIGHT * constants::TILE_SCALE) * self.height as f32,
        )
    }

    fn decode(&self, tile_id: u32) -> (u32, f32) {
        //let flip_d = (tile_id & constants::FLIP_DIAGONALLY_FLAG) == constants::FLIP_DIAGONALLY_FLAG;
        let flip_h =
            (tile_id & constants::FLIP_HORIZONTALLY_FLAG) == constants::FLIP_HORIZONTALLY_FLAG;
        let flip_v = (tile_id & constants::FLIP_VERTICALLY_FLAG) == constants::FLIP_VERTICALLY_FLAG;

        let new_tile_id = if flip_h | flip_v {
            tile_id & !constants::ALL_FLIP_FLAGS
        } else {
            tile_id
        };

        let rotate = match (flip_h, flip_v) {
            (true, false) => self.convert_angle_to_rad(90.0),
            (true, true) => self.convert_angle_to_rad(180.0),
            (false, true) => self.convert_angle_to_rad(270.0),
            (false, false) => 0.0,
        };

        (new_tile_id, rotate)
    }

    fn convert_angle_to_rad(&self, angle: f32) -> f32 {
        angle * (PI / 180.0)
    }
}

pub struct Layer {
    pub data: Vec<usize>,
}

impl Layer {
    pub fn new(text: String) -> Layer {
        Layer {
            data: text
                .replace("\n", "")
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect(),
        }
    }
}
