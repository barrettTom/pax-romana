use ggez::graphics::spritebatch::SpriteBatch;
use ggez::nalgebra::Point2;

use crate::animations::Animation;
use crate::constants;
use crate::entity::Operable;
use crate::tileset::Tileset;

#[derive(Debug, Clone)]
pub struct Cell {
    pub id: usize,
    pub animation: Animation,
    pub destination: Point2<f32>,
}

impl Operable for Cell {
    fn update(&mut self) {
        self.animation.update();
    }

    fn draw(&self, spritebatch: &mut SpriteBatch) {
        self.animation.draw(spritebatch, self.destination);
    }
}

impl Cell {
    pub fn new(text: &str, i: usize, tileset: &Tileset, dimensions: (usize, usize)) -> Cell {
        let id = text.parse::<usize>().unwrap();

        /*
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
        */

        let x = i as f32 % dimensions.0 as f32;
        let y = (i as f32 / dimensions.1 as f32).floor();
        //let offset = (constants::TILE_WIDTH / 2.0) * constants::TILE_SCALE;

        let destination = Point2::new(
            constants::TILE_WIDTH * constants::TILE_SCALE * x, //+ offset,
            constants::TILE_HEIGHT * constants::TILE_SCALE * y, //+ offset,
        );

        Cell {
            id,
            animation: tileset.get_animation(id),
            destination,
        }
    }
}
