use ggez::graphics::spritebatch::SpriteBatch;
use ggez::nalgebra::{distance, Point2};
use rand::Rng;
use std::f32::consts::PI;
use std::time::Instant;

use crate::animations::Animations;
use crate::constants;
use crate::entity::{Action, Entity, Operable};
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
        match self.behavior {
            Behavior::Wandering(destination) => self.move_torwards(destination),
            Behavior::Waiting(time) => self.wait(time),
        }
        self.entity.update();
        self.animations.update(&self.entity.action);
    }
}

impl NPC {
    pub fn new(tileset: &Tileset, spawn: Point2<f32>, map_dimensions: (f32, f32)) -> NPC {
        NPC {
            entity: Entity::new(spawn, map_dimensions),
            behavior: Behavior::Wandering(random_nearby_point(spawn, constants::WANDER_DISTANCE)),
            animations: Animations::new(tileset),
        }
    }

    fn move_torwards(&mut self, destination: Point2<f32>) {
        let position = self.entity.position;

        if distance(&position, &destination) < constants::GOAL_DISTANCE {
            self.entity.action = Action::IdleRight;
            self.behavior = Behavior::Waiting(Instant::now());
        } else if (position.x - destination.x).abs() < constants::GOAL_DISTANCE {
            if position.y > destination.y {
                self.entity.action = Action::MovingUp;
            } else {
                self.entity.action = Action::MovingDown;
            }
        } else if (position.y - destination.y).abs() < constants::GOAL_DISTANCE {
            if position.x > destination.x {
                self.entity.action = Action::MovingLeft;
            } else {
                self.entity.action = Action::MovingRight;
            }
        } else if position.x > destination.x {
            if position.y > destination.y {
                self.entity.action = Action::MovingUpLeft;
            } else {
                self.entity.action = Action::MovingDownLeft;
            }
        } else if position.x < destination.x {
            if position.y > destination.y {
                self.entity.action = Action::MovingUpRight;
            } else {
                self.entity.action = Action::MovingDownRight;
            }
        }
    }

    fn wait(&mut self, start: Instant) {
        if start.elapsed().as_secs() > constants::WAIT_TIME {
            self.behavior = Behavior::Wandering(random_nearby_point(
                self.entity.spawn,
                constants::WANDER_DISTANCE,
            ));
        }
    }

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
    Waiting(Instant),
    Wandering(Point2<f32>),
}

pub fn random_nearby_point(origin: Point2<f32>, within_radius: f32) -> Point2<f32> {
    let w = within_radius * rand::thread_rng().gen_range(0.0, 1.0);
    let t = 2.0 * PI * rand::thread_rng().gen_range(0.0, 1.0);
    Point2::new(origin.x + w * t.cos(), origin.y + w * t.sin())
}
