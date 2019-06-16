use ggez::filesystem::File;
use ggez::graphics::Rect;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub data: Vec<usize>,
}

impl Map {
    pub fn new(file: File) -> Map {
        let mut width = None;
        let mut height = None;
        let mut data: Option<Vec<usize>> = None;

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
                data = Some(
                    text.replace("\n", "")
                        .split(',')
                        .map(|s| s.parse().unwrap())
                        .collect(),
                );
            }
        }

        Map {
            data: data.unwrap(),
            width: width.unwrap(),
            height: height.unwrap(),
        }
    }
}

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

        for e in EventReader::new(BufReader::new(file)) {
            if let Ok(XmlEvent::StartElement { attributes, .. }) = e {
                for attribute in attributes {
                    match attribute.name.local_name.as_str() {
                        "columns" => columns = Some(attribute.value.parse::<usize>().unwrap()),
                        "tilewidth" => tile_width = Some(attribute.value.parse::<f32>().unwrap()),
                        "tileheight" => tile_height = Some(attribute.value.parse::<f32>().unwrap()),
                        _ => (),
                    }
                }
            }
        }

        let columns = columns.unwrap();

        let mut tiles = Vec::new();
        tiles.push(Rect::zero());

        for c in 0..columns {
            let x = c as f32 / columns as f32;
            let w = (c as f32 + 1.0) / columns as f32;
            tiles.push(Rect::new(x, 0.0, w, 1.0));
        }

        Tileset {
            tiles,
            tile_height: tile_height.unwrap(),
            tile_width: tile_width.unwrap(),
        }
    }
}
