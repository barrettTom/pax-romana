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
    state: PlayerState,
    source: Rect,
    timer: Instant,
    animation: Vec<(usize, Rect)>,
    animations: HashMap<PlayerState, Vec<(usize, Rect)>>,
    map_height: f32,
    map_width: f32,
}

impl Player {
    pub fn new(tileset: &Tileset, dimensions: (f32, f32)) -> Player {
        Player {
            position: Point2::new(0.0, 0.0),
            state: PlayerState::IdleLeft,
            source: Rect::zero(),
            timer: Instant::now(),
            animation: Vec::new(),
            animations: Player::build_animations(tileset),
            map_width: dimensions.0,
            map_height: dimensions.1,
        }
    }

    fn build_animations(tileset: &Tileset) -> HashMap<PlayerState, Vec<(usize, Rect)>> {
        let mut animations = HashMap::new();

        let mut source = tileset.get_tile_by_entity_keyframe("player-top", 0);
        source.h += tileset.get_tile_by_entity_keyframe("player-bottom", 0).h;
        animations.insert(PlayerState::IdleLeft, vec![(1, source)]);

        let mut moving = tileset.get_tile_by_entity_keyframe("player-top", 1);
        moving.h += tileset.get_tile_by_entity_keyframe("player-bottom", 1).h;

        animations.insert(PlayerState::MovingLeft, vec![(100, source), (100, moving)]);
        animations.insert(
            PlayerState::MovingUpLeft,
            vec![(100, source), (100, moving)],
        );
        animations.insert(
            PlayerState::MovingDownLeft,
            vec![(100, source), (100, moving)],
        );

        source = flip(source);
        moving = flip(moving);

        animations.insert(PlayerState::IdleRight, vec![(1, source)]);

        animations.insert(PlayerState::MovingRight, vec![(100, source), (100, moving)]);
        animations.insert(
            PlayerState::MovingUpRight,
            vec![(100, source), (100, moving)],
        );
        animations.insert(
            PlayerState::MovingDownRight,
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
            PlayerState::IdleLeft | PlayerState::IdleRight => (),
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
                _ => PlayerState::IdleLeft,
            },
            KeyCode::A => match original_state {
                PlayerState::MovingUpLeft => PlayerState::MovingUp,
                PlayerState::MovingDownLeft => PlayerState::MovingDown,
                _ => PlayerState::IdleLeft,
            },
            KeyCode::S => match original_state {
                PlayerState::MovingDownLeft => PlayerState::MovingLeft,
                PlayerState::MovingDownRight => PlayerState::MovingRight,
                _ => PlayerState::IdleRight,
            },
            KeyCode::D => match original_state {
                PlayerState::MovingUpRight => PlayerState::MovingUp,
                PlayerState::MovingDownRight => PlayerState::MovingDown,
                _ => PlayerState::IdleRight,
            },
            _ => original_state,
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
enum PlayerState {
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
