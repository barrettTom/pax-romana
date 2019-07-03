use ggez::graphics::Rect;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::time::Instant;

use crate::entity::Action;
use crate::tileset::Tileset;

#[derive(Clone)]
pub struct Animation {
    pub animation: Vec<(usize, Rect)>,
    pub animations: HashMap<Action, Vec<(usize, Rect)>>,
    pub timer: Instant,
    pub source: Rect,
}

impl Animation {
    pub fn new(tileset: &Tileset) -> Animation {
        let mut animations = HashMap::new();

        let mut source = tileset.get_tile_by_entity_keyframe("player-top", 0);
        source.h += tileset.get_tile_by_entity_keyframe("player-bottom", 0).h;
        animations.insert(Action::IdleLeft, vec![(1, source)]);

        let mut moving = tileset.get_tile_by_entity_keyframe("player-top", 1);
        moving.h += tileset.get_tile_by_entity_keyframe("player-bottom", 1).h;

        animations.insert(Action::MovingLeft, vec![(100, source), (100, moving)]);
        animations.insert(Action::MovingUpLeft, vec![(100, source), (100, moving)]);
        animations.insert(Action::MovingDownLeft, vec![(100, source), (100, moving)]);

        source = flip(source);
        moving = flip(moving);

        animations.insert(Action::IdleRight, vec![(1, source)]);

        animations.insert(Action::MovingRight, vec![(100, source), (100, moving)]);
        animations.insert(Action::MovingUpRight, vec![(100, source), (100, moving)]);
        animations.insert(Action::MovingDownRight, vec![(100, source), (100, moving)]);

        Animation {
            animations,
            source,
            timer: Instant::now(),
            animation: Vec::new(),
        }
    }

    pub fn update(&mut self, action: &Action) {
        self.animation = self.animations.get(&action).cloned().unwrap_or_default();
        let (source, timer) = next_source(self.source, &self.animation, self.timer);
        self.source = source;
        self.timer = timer;
    }
}

pub fn next_source(source: Rect, animation: &[(usize, Rect)], timer: Instant) -> (Rect, Instant) {
    if let Some(mut i) = animation.iter().position(|a| a.1 == source) {
        if timer.elapsed().as_millis() > animation[i].0 as u128 {
            i = if i == animation.len() - 1 { 0 } else { i + 1 };
            (animation[i].1, Instant::now())
        } else {
            (source, timer)
        }
    } else if !animation.is_empty() {
        (animation[0].1, timer)
    } else {
        (source, timer)
    }
}

pub fn convert_angle_to_rad(angle: f32) -> f32 {
    angle * (PI / 180.0)
}

pub fn flip(rect: Rect) -> Rect {
    let mut r = rect;
    r.x *= -1.0;
    r.x -= rect.w;
    r
}
