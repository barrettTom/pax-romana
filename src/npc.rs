use ggez::graphics::{spritebatch::SpriteBatch, DrawParam};
use ggez::nalgebra::{/*distance,*/ Point2, Vector2};
//use std::time::Instant;

use crate::animation::Animations;
use crate::constants;
use crate::entity::{Entity, Operable};
use crate::map::Map;
use crate::tileset::Tileset;

#[derive(Clone)]
pub struct NPC {
    entity: Entity,
    behavior: Behavior,
    animations: Animations,
}

impl Operable for NPC {
    fn draw(&self, spritebatch: &mut SpriteBatch) {
        spritebatch.add(
            DrawParam::default()
                .src(self.animations.current.current.source)
                .dest(self.entity.position)
                .scale(Vector2::new(constants::TILE_SCALE, constants::TILE_SCALE)),
        );
    }

    fn update(&mut self) {
        /*
        match self.behavior {
            Behavior::Wandering(destination) => self.wandering(destination),
            Behavior::Waiting(time) => (),
        }
        */
        self.animations.update(&self.entity.action);
    }
}

impl NPC {
    pub fn new(tileset: &Tileset, spawn: Point2<f32>, map_dimensions: (f32, f32)) -> NPC {
        NPC {
            entity: Entity::new(spawn, map_dimensions),
            behavior: Behavior::Wandering(None),
            animations: Animations::new(tileset),
        }
    }

    /*fn wandering(&mut self, destination: Option<Point2<f32>>) {
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
    }*/

    //fn waiting(&mut self) {}

    pub fn build_npcs(tileset: &Tileset, map: &Map) -> Vec<NPC> {
        let mut npcs = Vec::new();

        for (_name, position) in map.get_spawns() {
            npcs.push(NPC::new(tileset, position, map.get_dimensions()));
        }

        npcs
    }
}

#[derive(Clone)]
enum Behavior {
    //Waiting(Instant),
    Wandering(Option<Point2<f32>>),
}
