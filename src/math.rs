use std::f32::consts::PI;

pub fn convert_angle_to_rad(angle: f32) -> f32 {
    angle * (PI / 180.0)
}
