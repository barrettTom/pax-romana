use ggez::graphics::spritebatch::SpriteBatch;
use ggez::nalgebra::Point2;

use crate::constants;

pub trait Operable {
    fn update(&mut self);
    fn draw(&self, spritebatch: &mut SpriteBatch);
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub position: Point2<f32>,
    pub spawn: Point2<f32>,
    pub action: Action,
    map_dimensions: (f32, f32),
}

impl Entity {
    pub fn new(spawn: Point2<f32>, map_dimensions: (f32, f32)) -> Entity {
        Entity {
            spawn,
            action: Action::IdleLeft,
            position: spawn,
            map_dimensions,
        }
    }

    pub fn update(&mut self) {
        self.move_position();
    }

    fn move_position(&mut self) {
        match self.action {
            Action::MovingUp => self.position.y -= constants::PLAYER_SPEED,
            Action::MovingUpLeft => {
                self.position.x -= constants::PLAYER_SPEED / 2.0_f32.sqrt();
                self.position.y -= constants::PLAYER_SPEED / 2.0_f32.sqrt();
            }
            Action::MovingUpRight => {
                self.position.x += constants::PLAYER_SPEED / 2.0_f32.sqrt();
                self.position.y -= constants::PLAYER_SPEED / 2.0_f32.sqrt();
            }
            Action::MovingLeft => self.position.x -= constants::PLAYER_SPEED,
            Action::MovingDown => self.position.y += constants::PLAYER_SPEED,
            Action::MovingDownLeft => {
                self.position.x -= constants::PLAYER_SPEED / 2.0_f32.sqrt();
                self.position.y += constants::PLAYER_SPEED / 2.0_f32.sqrt();
            }
            Action::MovingDownRight => {
                self.position.x += constants::PLAYER_SPEED / 2.0_f32.sqrt();
                self.position.y += constants::PLAYER_SPEED / 2.0_f32.sqrt();
            }
            Action::MovingRight => self.position.x += constants::PLAYER_SPEED,
            Action::IdleLeft | Action::IdleRight => (),
        }

        let pixel_width = constants::TILE_WIDTH * constants::TILE_SCALE;
        let pixel_height = constants::TILE_HEIGHT * constants::TILE_SCALE;

        if self.position.x < 0.0 {
            self.position.x = 0.0;
        } else if self.position.x + pixel_height > self.map_dimensions.0 {
            self.position.x = self.map_dimensions.0 - pixel_width;
        }

        if self.position.y < 0.0 {
            self.position.y = 0.0;
        } else if self.position.y + pixel_height > self.map_dimensions.1 {
            self.position.y = self.map_dimensions.1 - pixel_height;
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum Action {
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
