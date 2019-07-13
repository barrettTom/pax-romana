use ggez::graphics::{spritebatch::SpriteBatch, DrawParam, Rect};
use ggez::nalgebra::{Point2, Vector2};
use std::f32::consts::PI;
use xml::reader::XmlEvent;

use crate::constants;
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

    pub fn draw(&self, spritebatch: &mut SpriteBatch, position: Point2<f32>) {
        let draw = match self.properties.visible {
            Some(draw) => draw,
            None => true,
        };

        if draw {
            spritebatch.add(
                DrawParam::default()
                    .src(self.source)
                    .rotation(self.properties.rotation)
                    .offset(Point2::new(0.5, 0.5))
                    .dest(position)
                    .scale(Vector2::new(constants::TILE_SCALE, constants::TILE_SCALE)),
            );
        }
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
    pub scramble_delay: Option<bool>,
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
        let spawn = XMLElements::get_attribute_value(&properties_elements, "spawn").ok();
        let visible = match XMLElements::get_attribute_value(&properties_elements, "visible") {
            Ok(visible) => visible.parse().ok(),
            Err(_) => None,
        };
        let delay = match XMLElements::get_attribute_value(&properties_elements, "delay") {
            Ok(delay) => delay.parse().ok(),
            Err(_) => None,
        };
        let scramble_delay =
            match XMLElements::get_attribute_value(&properties_elements, "scramble_delay") {
                Ok(scramble_delay) => scramble_delay.parse().ok(),
                Err(_) => None,
            };

        /*
        if scramble_delay {
            println!("in");
            let mut rng = rand::thread_rng();
            let d = delay.unwrap() as f32;
            let normal = Normal::new(d, d * 0.50).unwrap();
            delay = Some(normal.sample(&mut rng) as usize);
            println!("{:?}", delay);
        }
        */

        Properties {
            rotation: 0.0,
            entity,
            keyframe,
            delay,
            scramble_delay,
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
            scramble_delay: None,
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
