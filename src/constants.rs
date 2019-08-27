pub const TILE_WIDTH: f32 = 16.0;
pub const TILE_HEIGHT: f32 = 16.0;
pub const TILE_SCALE: f32 = 2.75;

pub const PLAYER_SPEED: f32 = 3.0;
pub const WANDER_DISTANCE: f32 = 200.0;
pub const INTERACT_DISTANCE: f32 = 30.0;
pub const WAIT_TIME: u64 = 3;

pub const FLOAT_PRECISION: f32 = 0.001;

pub const FLIP_H: usize = 0x8000_0000;
pub const FLIP_V: usize = 0x4000_0000;
pub const FLIP_D: usize = 0x2000_0000;
pub const FLIP_A: usize = FLIP_D | FLIP_H | FLIP_V;

use ggez::graphics::Color;
pub const PURPLE: Color = Color::new(0.4, 0.0, 0.2, 1.0);
pub const GOLD: Color = Color::new(1.0, 0.872, 0.376, 1.0);
pub const WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
