use ggez::event::KeyCode;
use ggez::graphics::{spritebatch::SpriteBatch, DrawParam};
use ggez::nalgebra::{Point2, Vector2};

use crate::constants;
use crate::tileset::Tileset;

pub struct Player {
    pub position: Point2<f32>,
    state: PlayerState,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: Point2::new(0.0, 0.0),
            state: PlayerState::Idle,
        }
    }

    pub fn draw(&self, spritebatch: &mut SpriteBatch, tileset: &Tileset) {
        let draw_param = DrawParam::default()
            .src(tileset.tiles[1])
            .dest(self.position)
            .scale(Vector2::new(constants::TILE_SCALE, constants::TILE_SCALE));

        spritebatch.add(draw_param);
    }

    pub fn update(&mut self) {
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

impl Default for Player {
    fn default() -> Self {
        Player::new()
    }
}

#[derive(Clone)]
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
