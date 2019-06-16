use ggez::filesystem::File;
use ggez::graphics::Rect;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

pub struct Tileset {
    pub tiles: Vec<Rect>,
    pub tile_width: f32,
    pub tile_height: f32,
}

impl Tileset {
    pub fn new(file: File) -> Tileset {
        let mut tile_width = None;
        let mut tile_height = None;
        let mut columns = None;
        let mut height = None;

        for e in EventReader::new(BufReader::new(file)) {
            if let Ok(XmlEvent::StartElement { attributes, .. }) = e {
                for attribute in attributes {
                    match attribute.name.local_name.as_str() {
                        "columns" => columns = Some(attribute.value.parse::<usize>().unwrap()),
                        "tilewidth" => tile_width = Some(attribute.value.parse::<f32>().unwrap()),
                        "tileheight" => tile_height = Some(attribute.value.parse::<f32>().unwrap()),
                        "height" => height = Some(attribute.value.parse::<usize>().unwrap()),
                        _ => (),
                    }
                }
            }
        }

        let columns = columns.unwrap();
        let tile_height = tile_height.unwrap();
        let tile_width = tile_width.unwrap();
        let height = height.unwrap();

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
