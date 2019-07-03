use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, spritebatch::SpriteBatch, DrawParam, FilterMode, Image, WrapMode};
use ggez::{filesystem, Context, GameResult};

use crate::camera::Camera;
use crate::entity::Operable;
use crate::map::Map;
use crate::npc::NPC;
use crate::player::Player;
use crate::tileset::Tileset;

pub struct State {
    map: Map,
    spritebatch: SpriteBatch,
    camera: Camera,
    player: Player,
    npcs: Vec<NPC>,
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
            map: map.clone(),
            spritebatch: SpriteBatch::new(image),
            camera: Camera::new(context, map_dimensions),
            player: Player::new(&tileset, map_dimensions),
            npcs: NPC::build_npcs(&tileset, &map),
        })
    }
}

impl EventHandler for State {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        self.map.update();
        self.player.update();
        for npc in self.npcs.iter_mut() {
            npc.update();
        }

        self.camera.give_center(self.player.get_position());
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, graphics::BLACK);

        self.map.draw(&mut self.spritebatch);
        self.player.draw(&mut self.spritebatch);
        for npc in self.npcs.iter_mut() {
            npc.draw(&mut self.spritebatch);
        }

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
