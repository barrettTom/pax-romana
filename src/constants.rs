pub const TILE_WIDTH: f32 = 16.0;
pub const TILE_HEIGHT: f32 = 16.0;
pub const TILE_SCALE: f32 = 3.0;

pub const PLAYER_SPEED: f32 = 5.0;
pub const ENTITY_SPEED: f32 = 2.5;
pub const WANDER_DISTANCE: f32 = 150.0;
pub const GOAL_DISTANCE: f32 = 6.0;

pub const FLIP_HORIZONTAL_FLAG: usize = 0x8000_0000;
pub const FLIP_VERTICAL_FLAG: usize = 0x4000_0000;
pub const FLIP_DIAGONAL_FLAG: usize = 0x2000_0000;
pub const ALL_FLIP_FLAGS: usize = FLIP_DIAGONAL_FLAG | FLIP_HORIZONTAL_FLAG | FLIP_VERTICAL_FLAG;
