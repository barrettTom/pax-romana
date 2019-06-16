use ggez::filesystem::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

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
