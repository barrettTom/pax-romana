use ggez::graphics::spritebatch::SpriteBatch;
use ggez::nalgebra::Point2;
use rand::Rng;
use std::f32::consts::PI;

use crate::animations::Animations;
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
        self.animations.draw(spritebatch, self.entity.position);
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

        for point in map.get_spawn_points("peasant") {
            npcs.push(NPC::new(tileset, point, map.get_dimensions()));
        }

        npcs
    }
}

#[derive(Clone)]
enum Behavior {
    //Waiting(Instant),
    Wandering(Option<Point2<f32>>),
}

pub fn random_nearby_point(origin: Point2<f32>, within_radius: f32) -> Point2<f32> {
    let w = within_radius * rand::thread_rng().gen_range(0.0, 1.0);
    let t = 2.0 * PI * rand::thread_rng().gen_range(0.0, 1.0);
    Point2::new(origin.x + w * t.cos(), origin.y + w * t.sin())
}
