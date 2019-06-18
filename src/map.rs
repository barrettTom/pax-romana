use ggez::filesystem::File;
use ggez::graphics::{spritebatch::SpriteBatch, DrawParam};
use ggez::nalgebra::{Point2, Vector2};
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

use crate::constants;
use crate::tileset::Tileset;

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub layers: Vec<Layer>,
}

impl Map {
    pub fn new(file: File) -> Map {
        let mut width = None;
        let mut height = None;
        let mut layers = Vec::new();

        for e in EventReader::new(BufReader::new(file)) {
            if let Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) = e
            {
                if name.local_name == "map" {
                    for attribute in attributes {
                        match attribute.name.local_name.as_str() {
                            "width" => width = Some(attribute.value.parse::<usize>().unwrap()),
                            "height" => height = Some(attribute.value.parse::<usize>().unwrap()),
                            _ => (),
                        }
                    }
                }
            } else if let Ok(XmlEvent::Characters(text)) = e {
                layers.push(Layer::new(text));
            }
        }

        Map {
            layers,
            width: width.unwrap(),
            height: height.unwrap(),
        }
    }

    pub fn draw(&mut self, spritebatch: &mut SpriteBatch, tileset: &Tileset) {
        for layer in self.layers.iter() {
            for x in 0..self.width {
                for y in 0..self.height {
                    let draw_param = DrawParam::default()
                        .src(tileset.tiles[layer.data[x + (y * self.height)]])
                        .dest(Point2::new(
                            tileset.tile_width * constants::TILE_SCALE * x as f32,
                            tileset.tile_height * constants::TILE_SCALE * y as f32,
                        ))
                        .scale(Vector2::new(constants::TILE_SCALE, constants::TILE_SCALE));

                    spritebatch.add(draw_param);
                }
            }
        }
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
