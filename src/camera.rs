use ggez::nalgebra::Point2;
use ggez::Context;

use crate::constants;

pub struct Camera {
    pub draw: Point2<f32>,
    window_height: f32,
    window_width: f32,
    map_height: f32,
    map_width: f32,
}

impl Camera {
    pub fn new(context: &mut Context, dimensions: (f32, f32)) -> Camera {
        Camera {
            draw: Point2::new(0.0, 0.0),
            window_height: context.conf.window_mode.height,
            window_width: context.conf.window_mode.width,
            map_width: dimensions.0,
            map_height: dimensions.1,
        }
    }

    pub fn give_center(&mut self, center: Point2<f32>) {
        self.draw.x = ((self.window_width / 2.0) - center.x) - (constants::TILE_WIDTH);
        self.draw.y = ((self.window_height / 2.0) - center.y) - (constants::TILE_HEIGHT);

        if self.draw.x > 0.0 {
            self.draw.x = 0.0;
        } else if self.draw.x - self.window_width < -1.0 * self.map_width {
            self.draw.x = -1.0 * (self.map_width - self.window_width);
        }

        if self.draw.y > 0.0 {
            self.draw.y = 0.0;
        } else if self.draw.y - self.window_height < -1.0 * self.map_height {
            self.draw.y = -1.0 * (self.map_height - self.window_height);
        }
    }
}
