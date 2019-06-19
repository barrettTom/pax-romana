use ggez::filesystem::File;
use ggez::graphics::Rect;

use crate::constants;
use crate::xmlelements::XMLElements;

pub struct Tileset {
    pub tiles: Vec<Rect>,
}

impl Tileset {
    pub fn new(file: File) -> Tileset {
        let elements = XMLElements::new(file);

        let height = elements.get_element_attribute("image", "height").unwrap();
        let columns = elements
            .get_element_attribute("tileset", "columns")
            .unwrap();

        let rows = height / (constants::TILE_HEIGHT as usize);

        let mut tiles = Vec::new();
        tiles.push(Rect::zero());

        for r in 0..rows {
            for c in 0..columns {
                let x = c as f32 / columns as f32;
                let y = r as f32 / rows as f32;
                let w = (c as f32 + 1.0) / columns as f32;
                let h = (r as f32 + 1.0) / rows as f32;
                tiles.push(Rect::new(x, y, w, h));
            }
        }

        Tileset { tiles }
    }
}
