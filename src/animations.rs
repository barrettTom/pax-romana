use ggez::graphics::{spritebatch::SpriteBatch, DrawParam, Rect};
use ggez::nalgebra::{Point2, Vector2};
use std::collections::HashMap;
use std::f32::consts::PI;
use std::time::Instant;

use crate::constants;
use crate::entity::Action;
use crate::tileset::Tileset;

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub source: Rect,
    pub delay: Option<usize>,
    pub rotation: f32,
}

impl Frame {
    pub fn new(source: Rect, delay: Option<usize>, rotation: f32) -> Frame {
        Frame {
            source,
            delay,
            rotation,
        }
    }
}

impl Default for Frame {
    fn default() -> Frame {
        Frame::new(Rect::zero(), None, 0.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Animation {
    pub frames: Vec<Frame>,
    pub timer: Instant,
    pub current: Frame,
}

impl Default for Animation {
    fn default() -> Animation {
        Animation::new(Frame::default())
    }
}

impl Animation {
    pub fn new(current: Frame) -> Animation {
        Animation {
            current,
            timer: Instant::now(),
            frames: Vec::new(),
        }
    }

    pub fn give_frames(&mut self, frames: Vec<Frame>) {
        self.frames = frames;
    }

    pub fn update(&mut self) {
        if let Some(mut i) = self.frames.iter().position(|a| a == &self.current) {
            if let Some(delay) = self.current.delay {
                if self.timer.elapsed().as_millis() > delay as u128 {
                    i = if i == self.frames.len() - 1 { 0 } else { i + 1 };
                    self.current = self.frames[i].clone();
                    self.timer = Instant::now();
                }
            }
        } else if !self.frames.is_empty() {
            self.current = self.frames[0].clone();
        }
    }

    pub fn draw(&self, spritebatch: &mut SpriteBatch, position: Point2<f32>) {
        spritebatch.add(
            DrawParam::default()
                .src(self.current.source)
                .dest(position)
                .scale(Vector2::new(constants::TILE_SCALE, constants::TILE_SCALE)),
        );
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

        let mut idle = tileset.get_frame_by_entity_keyframe("player-top", 0);
        idle.source.h += tileset
            .get_frame_by_entity_keyframe("player-bottom", 0)
            .source
            .h;

        let mut animation = Animation::new(idle.clone());
        animation.give_frames(vec![idle.clone()]);
        available.insert(Action::IdleLeft, animation.clone());

        let mut moving = tileset.get_frame_by_entity_keyframe("player-top", 1);
        moving.source.h += tileset
            .get_frame_by_entity_keyframe("player-bottom", 1)
            .source
            .h;

        animation.give_frames(vec![idle.clone(), moving.clone()]);
        available.insert(Action::MovingLeft, animation.clone());
        available.insert(Action::MovingUpLeft, animation.clone());
        available.insert(Action::MovingDownLeft, animation.clone());

        animation.give_frames(vec![flip(idle.clone())]);
        available.insert(Action::IdleRight, animation.clone());

        animation.give_frames(vec![flip(idle.clone()), flip(moving.clone())]);
        available.insert(Action::MovingRight, animation.clone());
        available.insert(Action::MovingUpRight, animation.clone());
        available.insert(Action::MovingDownRight, animation.clone());

        Animations {
            available,
            current: Animation::default(),
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

pub fn flip(frame: Frame) -> Frame {
    let mut f = frame.clone();
    f.source.x *= -1.0;
    f.source.x -= frame.source.w;
    f
}

pub fn convert_angle_to_rad(angle: f32) -> f32 {
    angle * (PI / 180.0)
}
