use ggez::nalgebra::Point2;

pub struct Camera {
    pub draw: Point2<f32>,
}

impl Camera {
    pub fn new(draw: Point2<f32>) -> Camera {
        Camera { draw }
    }

    pub fn give_center(&mut self, center: Point2<f32>) {}
}

impl Default for Camera {
    fn default() -> Camera {
        Camera::new(Point2::new(0.0, 0.0))
    }
}
