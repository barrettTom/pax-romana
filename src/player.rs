use ggez::event::KeyCode;
use ggez::graphics::{spritebatch::SpriteBatch, DrawParam};
use ggez::nalgebra::{Point2, Vector2};

use crate::animation::Animation;
use crate::constants;
use crate::entity::{Action, Entity, Operable};
use crate::tileset::Tileset;

pub struct Player {
    entity: Entity,
    animation: Animation,
}

impl Operable for Player {
    fn draw(&self, spritebatch: &mut SpriteBatch) {
        spritebatch.add(
            DrawParam::default()
                .src(self.animation.source)
                .dest(self.entity.position)
                .scale(Vector2::new(constants::TILE_SCALE, constants::TILE_SCALE)),
        );
    }

    fn update(&mut self) {
        self.move_position();
        self.animation.update(&self.entity.action);
    }
}

impl Player {
    pub fn new(tileset: &Tileset, dimensions: (f32, f32)) -> Player {
        Player {
            entity: Entity::new(Point2::new(0.0, 0.0), dimensions),
            animation: Animation::new(tileset),
        }
    }

    pub fn get_position(&self) -> Point2<f32> {
        self.entity.position
    }

    fn move_position(&mut self) {
        match self.entity.action {
            Action::MovingUp => self.entity.position.y -= constants::PLAYER_SPEED,
            Action::MovingUpLeft => {
                self.entity.position.x -= constants::PLAYER_SPEED / 2.0_f32.sqrt();
                self.entity.position.y -= constants::PLAYER_SPEED / 2.0_f32.sqrt();
            }
            Action::MovingUpRight => {
                self.entity.position.x += constants::PLAYER_SPEED / 2.0_f32.sqrt();
                self.entity.position.y -= constants::PLAYER_SPEED / 2.0_f32.sqrt();
            }
            Action::MovingLeft => self.entity.position.x -= constants::PLAYER_SPEED,
            Action::MovingDown => self.entity.position.y += constants::PLAYER_SPEED,
            Action::MovingDownLeft => {
                self.entity.position.x -= constants::PLAYER_SPEED / 2.0_f32.sqrt();
                self.entity.position.y += constants::PLAYER_SPEED / 2.0_f32.sqrt();
            }
            Action::MovingDownRight => {
                self.entity.position.x += constants::PLAYER_SPEED / 2.0_f32.sqrt();
                self.entity.position.y += constants::PLAYER_SPEED / 2.0_f32.sqrt();
            }
            Action::MovingRight => self.entity.position.x += constants::PLAYER_SPEED,
            Action::IdleLeft | Action::IdleRight => (),
        }

        let pixel_width = constants::TILE_WIDTH * constants::TILE_SCALE;
        let pixel_height = constants::TILE_HEIGHT * constants::TILE_SCALE;

        if self.entity.position.x < 0.0 {
            self.entity.position.x = 0.0;
        } else if self.entity.position.x + pixel_height > self.entity.map_dimensions.0 {
            self.entity.position.x = self.entity.map_dimensions.0 - pixel_width;
        }

        if self.entity.position.y < 0.0 {
            self.entity.position.y = 0.0;
        } else if self.entity.position.y + pixel_height > self.entity.map_dimensions.1 {
            self.entity.position.y = self.entity.map_dimensions.1 - pixel_height;
        }
    }

    pub fn give_key_down(&mut self, keycode: KeyCode) {
        let original_state = self.entity.action.clone();

        self.entity.action = match keycode {
            KeyCode::W => match original_state {
                Action::MovingLeft => Action::MovingUpLeft,
                Action::MovingRight => Action::MovingUpRight,
                _ => Action::MovingUp,
            },
            KeyCode::A => match original_state {
                Action::MovingUp => Action::MovingUpLeft,
                Action::MovingDown => Action::MovingDownLeft,
                _ => Action::MovingLeft,
            },
            KeyCode::S => match original_state {
                Action::MovingLeft => Action::MovingDownLeft,
                Action::MovingRight => Action::MovingDownRight,
                _ => Action::MovingDown,
            },
            KeyCode::D => match original_state {
                Action::MovingUp => Action::MovingUpRight,
                Action::MovingDown => Action::MovingDownRight,
                _ => Action::MovingRight,
            },
            _ => original_state,
        }
    }

    pub fn give_key_up(&mut self, keycode: KeyCode) {
        let original_state = self.entity.action.clone();

        self.entity.action = match keycode {
            KeyCode::W => match original_state {
                Action::MovingUpLeft => Action::MovingLeft,
                Action::MovingUpRight => Action::MovingRight,
                _ => Action::IdleLeft,
            },
            KeyCode::A => match original_state {
                Action::MovingUpLeft => Action::MovingUp,
                Action::MovingDownLeft => Action::MovingDown,
                _ => Action::IdleLeft,
            },
            KeyCode::S => match original_state {
                Action::MovingDownLeft => Action::MovingLeft,
                Action::MovingDownRight => Action::MovingRight,
                _ => Action::IdleRight,
            },
            KeyCode::D => match original_state {
                Action::MovingUpRight => Action::MovingUp,
                Action::MovingDownRight => Action::MovingDown,
                _ => Action::IdleRight,
            },
            _ => original_state,
        }
    }
}
