use ggez::graphics::{spritebatch::SpriteBatch, DrawParam};
use ggez::nalgebra::{Point2, Vector2};

use crate::constants;
use crate::tileset::Tileset;

pub struct Player {
    pub position: Point2<f32>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: Point2::new(0.0, 0.0),
        }
    }

    pub fn draw(&self, spritebatch: &mut SpriteBatch, tileset: &Tileset) {
        let draw_param = DrawParam::default()
            .src(tileset.tiles[1])
            .dest(self.position)
            .scale(Vector2::new(constants::TILE_SCALE, constants::TILE_SCALE));

        spritebatch.add(draw_param);
    }
}
