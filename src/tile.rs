use ggez::graphics::{spritebatch::SpriteBatch, DrawParam, Rect};
use ggez::nalgebra::{Point2, Vector2};

use crate::constants;
use crate::math::convert_angle_to_rad;
use crate::tileset::Tileset;

pub struct Tile {
    //id: u32,
    source: Rect,
    //animation: Option<Vec<(u32, Rect)>>,
    rotate: f32,
    destination: Point2<f32>,
}

impl Tile {
    pub fn new(text: &str, i: usize, tileset: &Tileset, width: usize, height: usize) -> Tile {
        let id = text.parse::<usize>().unwrap();

        //let flip_d = (id & constants::FLIP_DIAGONAL_FLAG) == constants::FLIP_DIAGONAL_FLAG;
        let flip_h = (id & constants::FLIP_HORIZONTAL_FLAG) == constants::FLIP_HORIZONTAL_FLAG;
        let flip_v = (id & constants::FLIP_VERTICAL_FLAG) == constants::FLIP_VERTICAL_FLAG;

        let id = if flip_h | flip_v {
            id & !constants::ALL_FLIP_FLAGS
        } else {
            id
        };

        let rotate = match (flip_h, flip_v) {
            (true, false) => convert_angle_to_rad(90.0),
            (true, true) => convert_angle_to_rad(180.0),
            (false, true) => convert_angle_to_rad(270.0),
            (false, false) => 0.0,
        };

        let x = i as f32 % width as f32;
        let y = (i as f32 / height as f32).floor();
        let offset = (constants::TILE_WIDTH / 2.0) * constants::TILE_SCALE;

        let destination = Point2::new(
            (constants::TILE_WIDTH * constants::TILE_SCALE * x) + offset,
            (constants::TILE_HEIGHT * constants::TILE_SCALE * y) + offset,
        );

        Tile {
            //id,
            source: tileset.tiles[id as usize],
            //animation: None,
            rotate,
            destination,
        }
    }

    pub fn draw(&self, spritebatch: &mut SpriteBatch) {
        let draw_param = DrawParam::default()
            .src(self.source)
            .rotation(self.rotate)
            .offset(Point2::new(0.5, 0.5))
            .dest(self.destination)
            .scale(Vector2::new(constants::TILE_SCALE, constants::TILE_SCALE));

        spritebatch.add(draw_param);
    }
}
