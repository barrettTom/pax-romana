use ggez::graphics::{spritebatch::SpriteBatch, DrawParam, Rect};
use ggez::nalgebra::{distance, Point2, Vector2};
use std::time::Instant;

use crate::constants;
use crate::map::Map;
use crate::math::random_nearby_point;
use crate::tileset::Tileset;

#[derive(Clone)]
pub struct Entity {
    behavior: Behavior,
    position: Point2<f32>,
    source: Rect,
    spawn: Point2<f32>,
}

impl Entity {
    pub fn new(tileset: &Tileset, spawn: Point2<f32>) -> Entity {
        let mut source = tileset.get_tile_by_entity_keyframe("player-top", 0);
        source.h += tileset.get_tile_by_entity_keyframe("player-bottom", 0).h;

        Entity {
            spawn,
            source,
            position: spawn,
            behavior: Behavior::Wandering(None),
        }
    }

    pub fn update(&mut self) {
        match self.behavior {
            Behavior::Wandering(destination) => self.wandering(destination),
            Behavior::Waiting(time) => (),
        }
    }

    pub fn wandering(&mut self, destination: Option<Point2<f32>>) {
        match destination {
            Some(destination) => {
                if distance(&self.position, &destination) < constants::GOAL_DISTANCE {
                    self.behavior = Behavior::Waiting(Instant::now())
                } else {
                    if self.position.x < destination.x {
                        self.position.x += constants::ENTITY_SPEED;
                    } else {
                        self.position.x -= constants::ENTITY_SPEED;
                    }
                    if self.position.y < destination.y {
                        self.position.y += constants::ENTITY_SPEED;
                    } else {
                        self.position.y -= constants::ENTITY_SPEED;
                    }
                }
            }
            None => {
                self.behavior = Behavior::Wandering(Some(random_nearby_point(
                    self.spawn,
                    constants::WANDER_DISTANCE,
                )))
            }
        }
    }

    pub fn waiting(&mut self) {}

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

#[derive(Clone)]
enum Behavior {
    Waiting(Instant),
    Wandering(Option<Point2<f32>>),
}
