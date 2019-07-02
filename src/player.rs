use ggez::event::KeyCode;
use ggez::graphics::{spritebatch::SpriteBatch, DrawParam, Rect};
use ggez::nalgebra::{Point2, Vector2};
use std::collections::HashMap;
use std::time::Instant;

use crate::constants;
use crate::math::{flip, next_source};
use crate::tileset::Tileset;

pub struct Player {
    pub position: Point2<f32>,
    state: AnimationState,
    source: Rect,
    timer: Instant,
    animation: Vec<(usize, Rect)>,
    animations: HashMap<AnimationState, Vec<(usize, Rect)>>,
    map_height: f32,
    map_width: f32,
}

impl Player {
    pub fn new(tileset: &Tileset, dimensions: (f32, f32)) -> Player {
        Player {
            position: Point2::new(0.0, 0.0),
            state: AnimationState::IdleLeft,
            source: Rect::zero(),
            timer: Instant::now(),
            animation: Vec::new(),
            animations: Player::build_animations(tileset),
            map_width: dimensions.0,
            map_height: dimensions.1,
        }
    }

    fn build_animations(tileset: &Tileset) -> HashMap<AnimationState, Vec<(usize, Rect)>> {
        let mut animations = HashMap::new();

        let mut source = tileset.get_tile_by_entity_keyframe("player-top", 0);
        source.h += tileset.get_tile_by_entity_keyframe("player-bottom", 0).h;
        animations.insert(AnimationState::IdleLeft, vec![(1, source)]);

        let mut moving = tileset.get_tile_by_entity_keyframe("player-top", 1);
        moving.h += tileset.get_tile_by_entity_keyframe("player-bottom", 1).h;

        animations.insert(AnimationState::MovingLeft, vec![(100, source), (100, moving)]);
        animations.insert(
            AnimationState::MovingUpLeft,
            vec![(100, source), (100, moving)],
        );
        animations.insert(
            AnimationState::MovingDownLeft,
            vec![(100, source), (100, moving)],
        );

        source = flip(source);
        moving = flip(moving);

        animations.insert(AnimationState::IdleRight, vec![(1, source)]);

        animations.insert(AnimationState::MovingRight, vec![(100, source), (100, moving)]);
        animations.insert(
            AnimationState::MovingUpRight,
            vec![(100, source), (100, moving)],
        );
        animations.insert(
            AnimationState::MovingDownRight,
            vec![(100, source), (100, moving)],
        );

        animations
    }

    pub fn draw(&self, spritebatch: &mut SpriteBatch) {
        let draw_param = DrawParam::default()
            .src(self.source)
            .dest(self.position)
            .scale(Vector2::new(constants::TILE_SCALE, constants::TILE_SCALE));

        spritebatch.add(draw_param);
    }

    pub fn update(&mut self) {
        self.move_position();

        self.animation = self
            .animations
            .get(&self.state)
            .cloned()
            .unwrap_or_default();
        let (source, timer) = next_source(self.source, &self.animation, self.timer);
        self.source = source;
        self.timer = timer;
    }

    fn move_position(&mut self) {
        match self.state {
            AnimationState::MovingUp => self.position.y -= constants::PLAYER_SPEED,
            AnimationState::MovingUpLeft => {
                self.position.x -= constants::PLAYER_SPEED / 2.0_f32.sqrt();
                self.position.y -= constants::PLAYER_SPEED / 2.0_f32.sqrt();
            }
            AnimationState::MovingUpRight => {
                self.position.x += constants::PLAYER_SPEED / 2.0_f32.sqrt();
                self.position.y -= constants::PLAYER_SPEED / 2.0_f32.sqrt();
            }
            AnimationState::MovingLeft => self.position.x -= constants::PLAYER_SPEED,
            AnimationState::MovingDown => self.position.y += constants::PLAYER_SPEED,
            AnimationState::MovingDownLeft => {
                self.position.x -= constants::PLAYER_SPEED / 2.0_f32.sqrt();
                self.position.y += constants::PLAYER_SPEED / 2.0_f32.sqrt();
            }
            AnimationState::MovingDownRight => {
                self.position.x += constants::PLAYER_SPEED / 2.0_f32.sqrt();
                self.position.y += constants::PLAYER_SPEED / 2.0_f32.sqrt();
            }
            AnimationState::MovingRight => self.position.x += constants::PLAYER_SPEED,
            AnimationState::IdleLeft | AnimationState::IdleRight => (),
        }

        let pixel_width = constants::TILE_WIDTH * constants::TILE_SCALE;
        let pixel_height = constants::TILE_HEIGHT * constants::TILE_SCALE;

        if self.position.x < 0.0 {
            self.position.x = 0.0;
        } else if self.position.x + pixel_height > self.map_width {
            self.position.x = self.map_width - pixel_width;
        }

        if self.position.y < 0.0 {
            self.position.y = 0.0;
        } else if self.position.y + pixel_height > self.map_height {
            self.position.y = self.map_height - pixel_height;
        }
    }

    pub fn give_key_down(&mut self, keycode: KeyCode) {
        let original_state = self.state.clone();

        self.state = match keycode {
            KeyCode::W => match original_state {
                AnimationState::MovingLeft => AnimationState::MovingUpLeft,
                AnimationState::MovingRight => AnimationState::MovingUpRight,
                _ => AnimationState::MovingUp,
            },
            KeyCode::A => match original_state {
                AnimationState::MovingUp => AnimationState::MovingUpLeft,
                AnimationState::MovingDown => AnimationState::MovingDownLeft,
                _ => AnimationState::MovingLeft,
            },
            KeyCode::S => match original_state {
                AnimationState::MovingLeft => AnimationState::MovingDownLeft,
                AnimationState::MovingRight => AnimationState::MovingDownRight,
                _ => AnimationState::MovingDown,
            },
            KeyCode::D => match original_state {
                AnimationState::MovingUp => AnimationState::MovingUpRight,
                AnimationState::MovingDown => AnimationState::MovingDownRight,
                _ => AnimationState::MovingRight,
            },
            _ => original_state,
        }
    }

    pub fn give_key_up(&mut self, keycode: KeyCode) {
        let original_state = self.state.clone();

        self.state = match keycode {
            KeyCode::W => match original_state {
                AnimationState::MovingUpLeft => AnimationState::MovingLeft,
                AnimationState::MovingUpRight => AnimationState::MovingRight,
                _ => AnimationState::IdleLeft,
            },
            KeyCode::A => match original_state {
                AnimationState::MovingUpLeft => AnimationState::MovingUp,
                AnimationState::MovingDownLeft => AnimationState::MovingDown,
                _ => AnimationState::IdleLeft,
            },
            KeyCode::S => match original_state {
                AnimationState::MovingDownLeft => AnimationState::MovingLeft,
                AnimationState::MovingDownRight => AnimationState::MovingRight,
                _ => AnimationState::IdleRight,
            },
            KeyCode::D => match original_state {
                AnimationState::MovingUpRight => AnimationState::MovingUp,
                AnimationState::MovingDownRight => AnimationState::MovingDown,
                _ => AnimationState::IdleRight,
            },
            _ => original_state,
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
enum AnimationState {
    IdleLeft,
    IdleRight,
    MovingUp,
    MovingDown,
    MovingLeft,
    MovingRight,
    MovingUpLeft,
    MovingUpRight,
    MovingDownLeft,
    MovingDownRight,
}
