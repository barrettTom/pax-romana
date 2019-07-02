use ggez::graphics::{spritebatch::SpriteBatch, DrawParam, Rect};
use ggez::nalgebra::{Point2, Vector2};
use std::time::Instant;

use crate::constants;
use crate::math::{convert_angle_to_rad, flip, next_source};
use crate::tileset::Tileset;

#[derive(Clone, Debug)]
pub struct Tile {
    pub id: usize,
    source: Rect,
    animation: Vec<(usize, Rect)>,
    timer: Instant,
    pub destination: Point2<f32>,
    rotation: f32,
}

impl Tile {
    pub fn new(text: &str, i: usize, tileset: &Tileset, width: usize, height: usize) -> Tile {
        let id = text.parse::<usize>().unwrap();

        let flip_d = (id & constants::FLIP_DIAGONAL_FLAG) == constants::FLIP_DIAGONAL_FLAG;
        let flip_h = (id & constants::FLIP_HORIZONTAL_FLAG) == constants::FLIP_HORIZONTAL_FLAG;
        let flip_v = (id & constants::FLIP_VERTICAL_FLAG) == constants::FLIP_VERTICAL_FLAG;

        let id = if flip_h | flip_v | flip_d {
            id & !constants::ALL_FLIP_FLAGS
        } else {
            id
        };

        let (source, rotation) = match (flip_d, flip_h, flip_v) {
            (true, true, true) => (flip(tileset.get(id)), convert_angle_to_rad(90.0)),
            (true, true, false) => (tileset.get(id), convert_angle_to_rad(90.0)),
            (true, false, true) => (tileset.get(id), convert_angle_to_rad(270.0)),
            //(true, false, false) => (),
            (false, true, true) => (tileset.get(id), convert_angle_to_rad(180.0)),
            (false, true, false) => (flip(tileset.get(id)), 0.0),
            //(false, false, true) => (),
            //(false, false, false) => (),
            _ => (tileset.get(id), 0.0),
        };

        let x = i as f32 % width as f32;
        let y = (i as f32 / height as f32).floor();
        let offset = (constants::TILE_WIDTH / 2.0) * constants::TILE_SCALE;

        let destination = Point2::new(
            (constants::TILE_WIDTH * constants::TILE_SCALE * x) + offset,
            (constants::TILE_HEIGHT * constants::TILE_SCALE * y) + offset,
        );

        Tile {
            id,
            source,
            animation: tileset.get_animation(id),
            timer: Instant::now(),
            destination,
            rotation,
        }
    }

    pub fn update(&mut self) {
        let (source, timer) = next_source(self.source, &self.animation, self.timer);
        self.source = source;
        self.timer = timer;
    }

    pub fn draw(&self, spritebatch: &mut SpriteBatch) {
        let draw_param = DrawParam::default()
            .src(self.source)
            .dest(self.destination)
            .offset(Point2::new(0.5, 0.5))
            .rotation(self.rotation)
            .scale(Vector2::new(constants::TILE_SCALE, constants::TILE_SCALE));

        spritebatch.add(draw_param);
    }
}
