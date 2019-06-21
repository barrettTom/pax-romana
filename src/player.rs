use ggez::event::KeyCode;
use ggez::graphics::{spritebatch::SpriteBatch, DrawParam, Rect};
use ggez::nalgebra::{Point2, Vector2};
use ggez::timer::check_update_time;
use ggez::Context;
use std::collections::HashMap;

use crate::constants;

pub struct Player {
    pub position: Point2<f32>,
    state: PlayerState,
    tile: Rect,
    animation: Vec<(u32, Rect)>,
    animations: HashMap<PlayerState, Vec<(u32, Rect)>>,
    map_height: f32,
    map_width: f32,
}

impl Player {
    pub fn new(dimensions: (f32, f32)) -> Player {
        let mut animations = HashMap::new();

        let mut idle = Vec::new();
        idle.push((1, Rect::new(0.0, 0.0, 0.2, 0.2)));
        idle.push((1, Rect::new(0.2, 0.0, 0.2, 0.2)));

        let mut moving_right = Vec::new();
        moving_right.push((1, Rect::new(0.4, 0.0, 0.2, 0.2)));
        moving_right.push((1, Rect::new(0.6, 0.0, 0.2, 0.2)));

        animations.insert(PlayerState::Idle, idle);
        animations.insert(PlayerState::MovingRight, moving_right);

        Player {
            position: Point2::new(0.0, 0.0),
            state: PlayerState::Idle,
            tile: Rect::zero(),
            animation: Vec::new(),
            animations,
            map_width: dimensions.0,
            map_height: dimensions.1,
        }
    }

    pub fn draw(&self, spritebatch: &mut SpriteBatch) {
        let draw_param = DrawParam::default()
            .src(self.tile)
            .dest(self.position)
            .scale(Vector2::new(constants::TILE_SCALE, constants::TILE_SCALE));

        spritebatch.add(draw_param);
    }

    pub fn update(&mut self, context: &mut Context) {
        self.move_position();
        self.find_tile(context);
    }

    fn move_position(&mut self) {
        match self.state {
            PlayerState::MovingUp => self.position.y -= constants::PLAYER_SPEED,
            PlayerState::MovingUpLeft => {
                self.position.x -= constants::PLAYER_SPEED / 2.0_f32.sqrt();
                self.position.y -= constants::PLAYER_SPEED / 2.0_f32.sqrt();
            }
            PlayerState::MovingUpRight => {
                self.position.x += constants::PLAYER_SPEED / 2.0_f32.sqrt();
                self.position.y -= constants::PLAYER_SPEED / 2.0_f32.sqrt();
            }
            PlayerState::MovingLeft => self.position.x -= constants::PLAYER_SPEED,
            PlayerState::MovingDown => self.position.y += constants::PLAYER_SPEED,
            PlayerState::MovingDownLeft => {
                self.position.x -= constants::PLAYER_SPEED / 2.0_f32.sqrt();
                self.position.y += constants::PLAYER_SPEED / 2.0_f32.sqrt();
            }
            PlayerState::MovingDownRight => {
                self.position.x += constants::PLAYER_SPEED / 2.0_f32.sqrt();
                self.position.y += constants::PLAYER_SPEED / 2.0_f32.sqrt();
            }
            PlayerState::MovingRight => self.position.x += constants::PLAYER_SPEED,
            PlayerState::Idle => (),
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

    fn find_tile(&mut self, context: &mut Context) {
        self.animation = match self.animations.get(&self.state) {
            Some(animation) => animation.to_vec(),
            None => self.animations.get(&PlayerState::Idle).unwrap().to_vec(),
        };

        let index = match self.animation.iter().position(|a| a.1 == self.tile) {
            Some(index) => {
                if check_update_time(context, self.animation[index].0) {
                    index + 1
                } else {
                    index
                }
            }
            None => 0,
        };

        if index == self.animation.len() {
            self.tile = self.animation[0].1;
        } else {
            self.tile = self.animation[index].1;
        }
    }

    pub fn give_key_down(&mut self, keycode: KeyCode) {
        let original_state = self.state.clone();

        self.state = match keycode {
            KeyCode::W => match original_state {
                PlayerState::MovingLeft => PlayerState::MovingUpLeft,
                PlayerState::MovingRight => PlayerState::MovingUpRight,
                _ => PlayerState::MovingUp,
            },
            KeyCode::A => match original_state {
                PlayerState::MovingUp => PlayerState::MovingUpLeft,
                PlayerState::MovingDown => PlayerState::MovingDownLeft,
                _ => PlayerState::MovingLeft,
            },
            KeyCode::S => match original_state {
                PlayerState::MovingLeft => PlayerState::MovingDownLeft,
                PlayerState::MovingRight => PlayerState::MovingDownRight,
                _ => PlayerState::MovingDown,
            },
            KeyCode::D => match original_state {
                PlayerState::MovingUp => PlayerState::MovingUpRight,
                PlayerState::MovingDown => PlayerState::MovingDownRight,
                _ => PlayerState::MovingRight,
            },
            _ => original_state,
        }
    }

    pub fn give_key_up(&mut self, keycode: KeyCode) {
        let original_state = self.state.clone();

        self.state = match keycode {
            KeyCode::W => match original_state {
                PlayerState::MovingUpLeft => PlayerState::MovingLeft,
                PlayerState::MovingUpRight => PlayerState::MovingRight,
                _ => PlayerState::Idle,
            },
            KeyCode::A => match original_state {
                PlayerState::MovingUpLeft => PlayerState::MovingUp,
                PlayerState::MovingDownLeft => PlayerState::MovingDown,
                _ => PlayerState::Idle,
            },
            KeyCode::S => match original_state {
                PlayerState::MovingDownLeft => PlayerState::MovingLeft,
                PlayerState::MovingDownRight => PlayerState::MovingRight,
                _ => PlayerState::Idle,
            },
            KeyCode::D => match original_state {
                PlayerState::MovingUpRight => PlayerState::MovingUp,
                PlayerState::MovingDownRight => PlayerState::MovingDown,
                _ => PlayerState::Idle,
            },
            _ => original_state,
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
enum PlayerState {
    Idle,
    MovingUp,
    MovingDown,
    MovingLeft,
    MovingRight,
    MovingUpLeft,
    MovingUpRight,
    MovingDownLeft,
    MovingDownRight,
}
