use ggez::filesystem::File;
use ggez::graphics::spritebatch::SpriteBatch;
use xml::reader::XmlEvent::Characters;

use crate::constants;
use crate::layer::Layer;
use crate::tileset::Tileset;
use crate::xmlelements::XMLElements;

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub layers: Vec<Layer>,
}

impl Map {
    pub fn new(file: File, tileset: &Tileset) -> Map {
        let elements = XMLElements::new(file);

        let width = elements
            .get_element_attribute("map", "width")
            .unwrap()
            .parse()
            .unwrap();
        let height = elements
            .get_element_attribute("map", "height")
            .unwrap()
            .parse()
            .unwrap();

        let layers = elements
            .events
            .iter()
            .filter_map(|e| {
                if let Characters(text) = e {
                    Some(Layer::new(text, tileset, width, height))
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

    pub fn draw(&self, spritebatch: &mut SpriteBatch) {
        for layer in self.layers.iter() {
            layer.draw(spritebatch);
        }
    }

    pub fn get_dimensions(&self) -> (f32, f32) {
        (
            (constants::TILE_WIDTH * constants::TILE_SCALE) * self.width as f32,
            (constants::TILE_HEIGHT * constants::TILE_SCALE) * self.height as f32,
        )
    }
}
