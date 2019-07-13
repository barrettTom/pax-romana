use ggez::graphics::Rect;
use std::f32::consts::PI;
use xml::reader::XmlEvent;

use crate::xmlelements::XMLElements;

#[derive(Clone, Debug, PartialEq)]
pub struct Tile {
    pub source: Rect,
    pub properties: Properties,
}

impl Tile {
    pub fn new(source: Rect, properties: Properties) -> Tile {
        Tile { source, properties }
    }
}

impl Default for Tile {
    fn default() -> Tile {
        Tile::new(Rect::zero(), Properties::default())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Properties {
    pub entity: Option<String>,
    pub rotation: f32,
    pub keyframe: Option<usize>,
    pub delay: Option<usize>,
    pub spawn: Option<String>,
    pub visible: Option<bool>,
}

impl Properties {
    pub fn new(properties_elements: Vec<XmlEvent>) -> Properties {
        let entity = match XMLElements::get_attribute_value(&properties_elements, "entity") {
            Ok(entity) => entity.parse().ok(),
            Err(_) => None,
        };
        let keyframe = match XMLElements::get_attribute_value(&properties_elements, "keyframe") {
            Ok(keyframe) => keyframe.parse().ok(),
            Err(_) => None,
        };
        let delay = match XMLElements::get_attribute_value(&properties_elements, "delay") {
            Ok(delay) => delay.parse().ok(),
            Err(_) => None,
        };
        let spawn = XMLElements::get_attribute_value(&properties_elements, "spawn").ok();
        let visible = match XMLElements::get_attribute_value(&properties_elements, "visible") {
            Ok(visible) => visible.parse().ok(),
            Err(_) => None,
        };

        Properties {
            rotation: 0.0,
            entity,
            keyframe,
            delay,
            spawn,
            visible,
        }
    }
}

impl Default for Properties {
    fn default() -> Properties {
        Properties {
            rotation: 0.0,
            entity: None,
            keyframe: None,
            delay: None,
            spawn: None,
            visible: None,
        }
    }
}

pub fn convert_angle_to_rad(angle: f32) -> f32 {
    angle * (PI / 180.0)
}

pub fn flip(tile: Tile) -> Tile {
    let mut t = tile.clone();
    t.source.x *= -1.0;
    t.source.x -= t.source.w;
    t
}

pub fn rotate(tile: Tile, angle: f32) -> Tile {
    let mut t = tile.clone();
    t.properties.rotation = convert_angle_to_rad(angle);
    t
}
