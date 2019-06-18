use ggez::nalgebra::Point2;
use ggez::Context;

pub struct Camera {
    pub draw: Point2<f32>,
    height: f32,
    width: f32,
}

impl Camera {
    pub fn new(context: &mut Context) -> Camera {
        Camera {
            draw: Point2::new(0.0, 0.0),
            height: context.conf.window_mode.height,
            width: context.conf.window_mode.width,
        }
    }

    pub fn give_center(&mut self, center: Point2<f32>) {
        self.draw.x = (self.width / 2.0) - center.x;
        self.draw.y = (self.height / 2.0) - center.y;
    }
}
