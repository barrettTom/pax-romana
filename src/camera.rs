use ggez::nalgebra::Point2;
use ggez::Context;

use crate::constants;

pub struct Camera {
    pub draw: Point2<f32>,
    window_dimensions: (f32, f32),
    map_dimensions: (f32, f32),
}

impl Camera {
    pub fn new(context: &mut Context, map_dimensions: (f32, f32)) -> Camera {
        Camera {
            draw: Point2::new(0.0, 0.0),
            window_dimensions: (
                context.conf.window_mode.width,
                context.conf.window_mode.height,
            ),
            map_dimensions,
        }
    }

    pub fn give_center(&mut self, center: Point2<f32>) {
        self.draw.x = ((self.window_dimensions.0 / 2.0) - center.x) - (constants::TILE_WIDTH);
        self.draw.y = ((self.window_dimensions.1 / 2.0) - center.y) - (constants::TILE_HEIGHT);

        if self.draw.x > 0.0 {
            self.draw.x = 0.0;
        } else if self.draw.x - self.window_dimensions.0 < -1.0 * self.map_dimensions.0 {
            self.draw.x = -1.0 * (self.map_dimensions.0 - self.window_dimensions.0);
        }

        if self.draw.y > 0.0 {
            self.draw.y = 0.0;
        } else if self.draw.y - self.window_dimensions.1 < -1.0 * self.map_dimensions.1 {
            self.draw.y = -1.0 * (self.map_dimensions.1 - self.window_dimensions.1);
        }
    }
}
