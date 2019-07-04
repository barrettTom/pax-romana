use ggez::graphics::spritebatch::SpriteBatch;
use ggez::nalgebra::Point2;

use crate::animations::{Animation, Frame};
use crate::constants;
use crate::entity::Operable;
use crate::math::{convert_angle_to_rad, flip};
use crate::tileset::Tileset;

#[derive(Debug, Clone)]
pub struct Tile {
    pub id: usize,
    pub animation: Animation,
    pub destination: Point2<f32>,
}

impl Operable for Tile {
    fn update(&mut self) {
        self.animation.update();
    }

    fn draw(&self, spritebatch: &mut SpriteBatch) {
        self.animation.draw(spritebatch, self.destination);
        /*
            DrawParam::default()
                .src(self.animation.current.source)
                .dest(self.destination)
                .offset(Point2::new(0.5, 0.5))
                //.rotation(self.rotation)
                .scale(Vector2::new(constants::TILE_SCALE, constants::TILE_SCALE)),
        );
        */
    }
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
        //let offset = (constants::TILE_WIDTH / 2.0) * constants::TILE_SCALE;

        let destination = Point2::new(
            constants::TILE_WIDTH * constants::TILE_SCALE * x, //+ offset,
            constants::TILE_HEIGHT * constants::TILE_SCALE * y, //+ offset,
        );

        let mut animation = Animation::new(Frame::new(source, None, rotation));
        animation.give_frames(tileset.get_frames(id));

        Tile {
            id,
            animation,
            destination,
        }
    }
}
