use ggez::event::KeyCode;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::nalgebra::Point2;

use crate::animations::Animations;
use crate::entity::{Action, Entity, Operable};
use crate::tileset::Tileset;

pub struct Player {
    entity: Entity,
    animations: Animations,
}

impl Operable for Player {
    fn draw(&self, spritebatch: &mut SpriteBatch) {
        self.animations.draw(spritebatch, self.get_position());
    }

    fn update(&mut self) {
        self.entity.update();
        self.animations.update(&self.entity.action);
    }
}

impl Player {
    pub fn new(tileset: &Tileset, spawn: Point2<f32>, map_dimensions: (f32, f32)) -> Player {
        Player {
            entity: Entity::new(spawn, map_dimensions),
            animations: Animations::new(tileset),
        }
    }

    pub fn get_position(&self) -> Point2<f32> {
        self.entity.position
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
