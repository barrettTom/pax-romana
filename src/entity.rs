use ggez::graphics::spritebatch::SpriteBatch;
use ggez::nalgebra::Point2;

pub trait Operable {
    fn update(&mut self);
    fn draw(&self, spritebatch: &mut SpriteBatch);
}

#[derive(Clone)]
pub struct Entity {
    pub position: Point2<f32>,
    pub spawn: Point2<f32>,
    pub action: Action,
    pub map_dimensions: (f32, f32),
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
}

#[derive(Clone, Hash, Eq, PartialEq)]
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
