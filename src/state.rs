use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, spritebatch::SpriteBatch, DrawParam, FilterMode, Image};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{filesystem, Context, GameResult};

use crate::constants;
use crate::map::Map;
use crate::tileset::Tileset;

pub struct State {
    map: Map,
    tileset: Tileset,
    spritebatch: SpriteBatch,
    camera_point: (f32, f32),
}

impl State {
    pub fn new(context: &mut Context) -> GameResult<State> {
        let mut image = Image::new(context, "/tileset.png")?;
        image.set_filter(FilterMode::Nearest);

        Ok(State {
            map: Map::new(filesystem::open(context, "/map.tmx")?),
            tileset: Tileset::new(filesystem::open(context, "/tileset.tsx")?),
            spritebatch: SpriteBatch::new(image),
            camera_point: (0.0, 0.0),
        })
    }
}

impl EventHandler for State {
    fn update(&mut self, _: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, graphics::BLACK);

        for layer in self.map.layers.iter() {
            for x in 0..self.map.width {
                for y in 0..self.map.height {
                    let draw_param = DrawParam::default()
                        .src(self.tileset.tiles[layer.data[x + (y * self.map.height)]])
                        .dest(Point2::new(
                            self.tileset.tile_width * constants::TILE_SCALE * x as f32,
                            self.tileset.tile_height * constants::TILE_SCALE * y as f32,
                        ))
                        .scale(Vector2::new(constants::TILE_SCALE, constants::TILE_SCALE));

                    self.spritebatch.add(draw_param);
                }
            }
        }

        let draw_param =
            DrawParam::default().dest(Point2::new(self.camera_point.0, self.camera_point.1));

        graphics::draw(context, &self.spritebatch, draw_param)?;
        self.spritebatch.clear();

        graphics::present(context)?;
        Ok(())
    }

    fn key_down_event(&mut self, _: &mut Context, keycode: KeyCode, _: KeyMods, _: bool) {
        match keycode {
            KeyCode::W => self.camera_point.1 += constants::CAMERA_MOVE,
            KeyCode::A => self.camera_point.0 += constants::CAMERA_MOVE,
            KeyCode::S => self.camera_point.1 -= constants::CAMERA_MOVE,
            KeyCode::D => self.camera_point.0 -= constants::CAMERA_MOVE,
            _ => (),
        }
    }
}
