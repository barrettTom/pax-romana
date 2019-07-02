use ggez::graphics::{spritebatch::SpriteBatch, DrawParam, Rect};
use ggez::nalgebra::{Point2, Vector2};

use crate::constants;
use crate::map::Map;
use crate::tileset::Tileset;

#[derive(Clone)]
pub struct Entity {
    position: Point2<f32>,
    source: Rect,
    spawn: Point2<f32>,
}

impl Entity {
    pub fn new(tileset: &Tileset, spawn: Point2<f32>) -> Entity {
        let mut source = tileset.get_tile_by_entity_keyframe("player-top", 0);
        source.h += tileset.get_tile_by_entity_keyframe("player-bottom", 0).h;

        Entity {
            position: spawn,
            source,
            spawn,
        }
    }

    pub fn draw(&self, spritebatch: &mut SpriteBatch) {
        let draw_param = DrawParam::default()
            .src(self.source)
            .dest(self.position)
            .scale(Vector2::new(constants::TILE_SCALE, constants::TILE_SCALE));

        spritebatch.add(draw_param);
    }

    pub fn build_entities(tileset: &Tileset, map: &Map) -> Vec<Entity> {
        let mut entities = Vec::new();

        for (_name, position) in map.get_spawns() {
            entities.push(Entity::new(tileset, position));
        }

        entities
    }
}
