use ggez::graphics::{spritebatch::SpriteBatch, DrawParam};
use ggez::nalgebra::{Point2, Vector2};
use std::collections::HashMap;
use std::time::Instant;

use crate::constants;
use crate::entity::Action;
use crate::tile::Tile;
use crate::tileset::Tileset;

#[derive(Debug, Clone, PartialEq)]
pub struct Animation {
    pub frames: Vec<Tile>,
    pub timer: Instant,
    pub current: Tile,
}

impl Animation {
    pub fn new(frames: Vec<Tile>) -> Animation {
        Animation {
            current: frames[0].clone(),
            timer: Instant::now(),
            frames,
        }
    }

    pub fn give_frames(&mut self, frames: Vec<Tile>) {
        self.frames = frames;
    }

    pub fn update(&mut self) {
        if let Some(mut i) = self.frames.iter().position(|a| a == &self.current) {
            if let Some(delay) = self.current.properties.delay {
                if self.timer.elapsed().as_millis() > delay as u128 {
                    i = if i == self.frames.len() - 1 { 0 } else { i + 1 };
                    self.current = self.frames[i].clone();
                    self.timer = Instant::now();
                }
            }
        } else {
            self.current = self.frames[0].clone();
        }
    }

    pub fn draw(&self, spritebatch: &mut SpriteBatch, position: Point2<f32>) {
        if self.current.properties.visible.is_none() {
            spritebatch.add(
                DrawParam::default()
                    .src(self.current.source)
                    .rotation(self.current.properties.rotation)
                    .offset(Point2::new(0.5, 0.5))
                    .dest(position)
                    .scale(Vector2::new(constants::TILE_SCALE, constants::TILE_SCALE)),
            );
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Animations {
    pub available: HashMap<Action, Animation>,
    pub current: Animation,
}

impl Animations {
    pub fn new(tileset: &Tileset) -> Animations {
        let mut available = HashMap::new();

        let mut idle = tileset.get_tile_by_entity_keyframe("player-top", 0);
        idle.source.h *= 2.0;

        let animation = Animation::new(vec![idle.clone()]);
        available.insert(Action::IdleLeft, animation.clone());

        let mut moving = tileset.get_tile_by_entity_keyframe("player-top", 1);
        moving.source.h *= 2.0;

        let animation = Animation::new(vec![idle.clone(), moving.clone()]);
        available.insert(Action::MovingLeft, animation.clone());
        available.insert(Action::MovingUpLeft, animation.clone());
        available.insert(Action::MovingDownLeft, animation.clone());

        idle.flip();
        moving.flip();

        let animation = Animation::new(vec![idle.clone()]);
        available.insert(Action::IdleRight, animation.clone());

        let animation = Animation::new(vec![idle.clone(), moving.clone()]);
        available.insert(Action::MovingRight, animation.clone());
        available.insert(Action::MovingUpRight, animation.clone());
        available.insert(Action::MovingDownRight, animation.clone());

        Animations {
            available,
            current: animation,
        }
    }

    pub fn update(&mut self, action: &Action) {
        if let Some(animation) = self.available.get(&action).cloned() {
            self.current.give_frames(animation.frames);
        }
        self.current.update();
    }

    pub fn draw(&self, spritebatch: &mut SpriteBatch, position: Point2<f32>) {
        self.current.draw(spritebatch, position)
    }
}
