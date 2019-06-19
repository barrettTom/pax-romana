use ggez::filesystem::File;
use ggez::graphics::{spritebatch::SpriteBatch, DrawParam};
use ggez::nalgebra::{Point2, Vector2};
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
                    let draw_param = DrawParam::default()
                        .src(tileset.tiles[layer.data[x + (y * self.height)]])
                        .dest(Point2::new(
                            constants::TILE_WIDTH * constants::TILE_SCALE * x as f32,
                            constants::TILE_HEIGHT * constants::TILE_SCALE * y as f32,
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
