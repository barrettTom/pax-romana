use ggez::filesystem::File;
use ggez::graphics::Rect;
use std::collections::HashMap;

use crate::constants;
use crate::xmlelements::{Property, XMLElements};

pub struct Tileset {
    tiles: HashMap<usize, Rect>,
    properties: HashMap<usize, Property>,
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
        tiles.insert(0, Rect::zero());

        let w = 1.0 / columns as f32;
        let h = 1.0 / rows as f32;
        let mut key = 1;
        for r in 0..rows {
            for c in 0..columns {
                let x = c as f32 / columns as f32;
                let y = r as f32 / rows as f32;
                tiles.insert(key, Rect::new(x, y, w, h));
                key += 1;
            }
        }

        let mut properties = HashMap::new();

        for tile_element in elements.get_elements("tile") {
            let tile_id = XMLElements::get_attribute(&tile_element, "id")
                .unwrap()
                .parse()
                .unwrap();

            let property_elements = elements.get_children(&tile_element, "property");

            properties.insert(tile_id, Property::new(property_elements));
        }

        Tileset { tiles, properties }
    }

    pub fn get(&self, id: usize) -> Rect {
        *self.tiles.get(&id).unwrap()
    }
}
