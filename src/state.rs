use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, spritebatch::SpriteBatch, DrawParam, FilterMode, Image, Text};
use ggez::nalgebra::Point2;
use ggez::{filesystem, Context, GameResult};

use crate::camera::Camera;
use crate::constants;
use crate::map::Map;
use crate::tileset::Tileset;

pub struct State {
    map: Map,
    tileset: Tileset,
    spritebatch: SpriteBatch,
    camera: Camera,
    player_position: Point2<f32>,
}

impl State {
    pub fn new(context: &mut Context) -> GameResult<State> {
        let mut image = Image::new(context, "/tileset.png")?;
        image.set_filter(FilterMode::Nearest);

        Ok(State {
            map: Map::new(filesystem::open(context, "/map.tmx")?),
            tileset: Tileset::new(filesystem::open(context, "/tileset.tsx")?),
            spritebatch: SpriteBatch::new(image),
            camera: Camera::default(),
            player_position: Point2::new(0.0, 0.0),
        })
    }
}

impl EventHandler for State {
    fn update(&mut self, _: &mut Context) -> GameResult {
        self.camera.give_center(self.player_position);
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, graphics::BLACK);

        self.map.draw(&mut self.spritebatch, &self.tileset);

        graphics::draw(
            context,
            &self.spritebatch,
            DrawParam::default().dest(self.camera.draw),
        )?;

        graphics::draw(
            context,
            &Text::new("@"),
            DrawParam::default().dest(self.player_position),
        )?;

        self.spritebatch.clear();

        graphics::present(context)?;

        Ok(())
    }

    fn key_down_event(&mut self, _: &mut Context, keycode: KeyCode, _: KeyMods, _: bool) {
        match keycode {
            KeyCode::W => self.player_position.y -= constants::PLAYER_SPEED,
            KeyCode::A => self.player_position.x -= constants::PLAYER_SPEED,
            KeyCode::S => self.player_position.y += constants::PLAYER_SPEED,
            KeyCode::D => self.player_position.x += constants::PLAYER_SPEED,
            _ => (),
        }
    }
}
