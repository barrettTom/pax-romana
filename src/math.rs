use ggez::graphics::Rect;
use std::f32::consts::PI;
use std::time::Instant;

pub fn convert_angle_to_rad(angle: f32) -> f32 {
    angle * (PI / 180.0)
}

pub fn flip(rect: Rect) -> Rect {
    let mut r = rect;
    r.x *= -1.0;
    r.x -= rect.w;
    r
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
