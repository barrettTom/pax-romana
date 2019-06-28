use ggez::graphics::Rect;
use std::f32::consts::PI;
use std::time::Instant;

pub fn convert_angle_to_rad(angle: f32) -> f32 {
    angle * (PI / 180.0)
}

pub fn next_source(
    source: Rect,
    animation: &Option<Vec<(usize, Rect)>>,
    timer: Instant,
) -> (Rect, Instant) {
    if let Some(animation) = animation {
        if let Some(mut i) = animation.iter().position(|a| a.1 == source) {
            if timer.elapsed().as_millis() > animation[i].0 as u128 {
                i = if i == animation.len() - 1 { 0 } else { i + 1 };
                return (animation[i].1, Instant::now());
            }
        }
    }
    (source, timer)
}
