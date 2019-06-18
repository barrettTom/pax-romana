use ggez::filesystem::File;
use ggez::graphics::Rect;

use crate::xmlelements::XMLElements;

pub struct Tileset {
    pub tiles: Vec<Rect>,
    pub tile_width: f32,
    pub tile_height: f32,
}

impl Tileset {
    pub fn new(file: File) -> Tileset {
        let elements = XMLElements::new(file);

        let columns = elements
            .get_element_attribute("tileset", "columns")
            .unwrap();
        let height = elements.get_element_attribute("image", "height").unwrap();
        let tile_width = elements
            .get_element_attribute("tileset", "tilewidth")
            .unwrap() as f32;
        let tile_height = elements
            .get_element_attribute("tileset", "tileheight")
            .unwrap() as f32;

        let rows = height / (tile_height as usize);

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

        Tileset {
            tiles,
            tile_height,
            tile_width,
        }
    }
}
