use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, spritebatch::SpriteBatch, DrawParam, FilterMode, Image, WrapMode};
use ggez::{filesystem, Context, GameResult};

use crate::camera::Camera;
use crate::map::Map;
use crate::player::Player;
use crate::tileset::Tileset;

pub struct State {
    map: Map,
    spritebatch: SpriteBatch,
    camera: Camera,
    player: Player,
}

impl State {
    pub fn new(context: &mut Context) -> GameResult<State> {
        let mut image = Image::new(context, "/tileset.png")?;
        image.set_filter(FilterMode::Nearest);
        image.set_wrap(WrapMode::Mirror, WrapMode::Mirror);

        let tileset = Tileset::new(filesystem::open(context, "/tileset.tsx")?);

        let map = Map::new(filesystem::open(context, "/map.tmx")?, &tileset);
        let map_dimensions = map.get_dimensions();

        Ok(State {
            map,
            spritebatch: SpriteBatch::new(image),
            camera: Camera::new(context, map_dimensions),
            player: Player::new(map_dimensions),
        })
    }
}

impl EventHandler for State {
    fn update(&mut self, context: &mut Context) -> GameResult {
        self.map.update();
        self.player.update(context);
        self.camera.give_center(self.player.position);
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, graphics::BLACK);

        self.map.draw(&mut self.spritebatch);
        self.player.draw(&mut self.spritebatch);

        graphics::draw(
            context,
            &self.spritebatch,
            DrawParam::default().dest(self.camera.draw),
        )?;

        self.spritebatch.clear();

        graphics::present(context)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        context: &mut Context,
        keycode: KeyCode,
        _: KeyMods,
        repeat: bool,
    ) {
        if !repeat {
            match keycode {
                KeyCode::Q => context.continuing = false,
                _ => self.player.give_key_down(keycode),
            }
        }
    }

    fn key_up_event(&mut self, _: &mut Context, keycode: KeyCode, _: KeyMods) {
        self.player.give_key_up(keycode)
    }
}
