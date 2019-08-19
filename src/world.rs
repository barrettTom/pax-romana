use ggez::event::KeyCode;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::nalgebra::distance;
use ggez::{filesystem, Context};

use crate::constants;
use crate::dialogbox::DialogTree;
use crate::entity::Operable;
use crate::map::Map;
use crate::npc::{Character, NPC};
use crate::player::Player;
use crate::tileset::Tileset;

#[derive(Clone)]
pub struct World {
    map: Map,
    pub player: Player,
    npcs: Vec<NPC>,
}

impl Operable for World {
    fn update(&mut self) {
        self.map.update();
        self.player.update();
        for npc in self.npcs.iter_mut() {
            npc.update();
        }
    }

    fn draw(&self, spritebatch: &mut SpriteBatch) {
        self.map.draw(spritebatch);
        self.player.draw(spritebatch);
        for npc in self.npcs.iter() {
            npc.draw(spritebatch);
        }
    }
}

impl World {
    pub fn new(context: &mut Context) -> World {
        let tileset = Tileset::new(filesystem::open(context, "/tileset.tsx").unwrap());
        let map = Map::new(filesystem::open(context, "/map.tmx").unwrap(), &tileset);

        World {
            map: map.clone(),
            player: Player::new(
                &tileset,
                map.get_spawn_points(Character::Player)[0],
                map.get_dimensions(),
            ),
            npcs: NPC::build_npcs(context, &tileset, &map),
        }
    }

    pub fn get_dialogtree(&mut self) -> Option<DialogTree> {
        let player_position = self.player.entity.position;
        if let Some(npc) = self.npcs.iter_mut().find(|npc| {
            constants::INTERACT_DISTANCE > distance(&player_position, &npc.entity.position)
        }) {
            Some(npc.get_dialogtree())
        } else {
            None
        }
    }

    pub fn give_key_up(&mut self, keycode: KeyCode) {
        self.player.give_key_up(keycode);
    }

    pub fn give_key_down(&mut self, keycode: KeyCode) {
        self.player.give_key_down(keycode);
    }

    pub fn get_dimensions(&self) -> (f32, f32) {
        self.map.get_dimensions()
    }
}
