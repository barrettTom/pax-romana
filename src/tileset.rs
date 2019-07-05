use ggez::filesystem::File;
use ggez::graphics::Rect;
use std::collections::HashMap;

use crate::animations::{Animation, Frame};
use crate::constants;
use crate::property::Property;
use crate::xmlelements::XMLElements;

pub struct Tileset {
    tiles: HashMap<usize, Frame>,
    properties: Vec<Property>,
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
        tiles.insert(0, Frame::default());

        let w = 1.0 / columns as f32;
        let h = 1.0 / rows as f32;
        let mut key = 1;
        for r in 0..rows {
            for c in 0..columns {
                let x = c as f32 / columns as f32;
                let y = r as f32 / rows as f32;
                tiles.insert(key, Frame::new(Rect::new(x, y, w, h), None, 0.0));
                key += 1;
            }
        }

        let mut properties = Vec::new();

        for tile_element in elements.get_elements("tile") {
            let tile_id = XMLElements::get_attribute(&tile_element, "id")
                .unwrap()
                .parse::<usize>()
                .unwrap()
                + 1;

            let property_elements = elements.get_children(&tile_element, "property");

            properties.push(Property::new(tile_id, property_elements));
        }

        let invisible: Vec<usize> = properties
            .iter()
            .filter(|p| p.visible == Some(false))
            .map(|p| p.tile_id)
            .collect();

        for i in invisible {
            *tiles.get_mut(&i).unwrap() = Frame::default();
        }

        for tile in &tiles {}

        Tileset { tiles, properties }
    }

    pub fn get_spawn_tiles(&self) -> Vec<(String, usize)> {
        self.properties
            .clone()
            .into_iter()
            .filter(|p| p.spawn.is_some())
            .map(|p| (p.spawn.unwrap(), p.tile_id))
            .collect()
    }

    /*
    pub fn get_frames(&self, tile_id: usize) -> Vec<Frame> {
        if let Some(property) = self.properties.iter().find(|p| p.tile_id == tile_id) {
            self.properties
                .clone()
                .into_iter()
                .filter(|p| p.entity == property.entity && p.entity.is_some())
                .map(|p| Frame::new(*self.tiles.get(&p.tile_id).unwrap(), p.delay, 0.0))
                .collect()
        } else {
            Vec::new()
        }
    }
    */

    pub fn get_animation(&self, tile_id: usize) -> Animation {
        Animation::default()
    }

    pub fn get_frame_by_entity_keyframe(&self, entity: &str, keyframe: usize) -> Frame {
        let tile_id = &self
            .properties
            .iter()
            .find(|p| p.entity == Some(entity.to_string()) && Some(keyframe) == p.keyframe)
            .unwrap()
            .tile_id;

        /*
        let delay = self
            .properties
            .iter()
            .find(|p| p.tile_id == *tile_id && p.delay.is_some())
            .unwrap()
            .delay;

        Frame::new(*source, delay, 0.0)
        */

        self.tiles.get(tile_id).unwrap().clone()
    }
}
