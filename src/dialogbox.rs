use ggez::conf::Conf;
use ggez::graphics::{
    self, DrawMode, DrawParam, Font, Mesh, MeshBuilder, Rect, Scale, Text, TextFragment,
};
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};

use crate::constants;

pub struct DialogBox {
    mesh: Mesh,
    text: Text,
    conf: Conf,
    pub visible: bool,
}

impl DialogBox {
    pub fn new(context: &mut Context) -> DialogBox {
        let conf = Conf::new();
        let font = Font::new(context, "/fonts/SONORM__.ttf").unwrap();

        DialogBox {
            text: Text::new(
                TextFragment::new("Ave !")
                    .font(font)
                    .scale(Scale::uniform(40.0)),
            ),
            mesh: MeshBuilder::new()
                .rectangle(
                    DrawMode::fill(),
                    Rect::new(
                        conf.window_mode.width * 0.10,
                        2.5 * conf.window_mode.height / 4.0,
                        conf.window_mode.width * 0.80,
                        conf.window_mode.height / 4.0,
                    ),
                    constants::PURPLE,
                )
                .build(context)
                .unwrap(),
            visible: false,
            conf,
        }
    }

    pub fn draw(&self, context: &mut Context) -> GameResult {
        if self.visible {
            graphics::draw(context, &self.mesh, DrawParam::default())?;
            graphics::draw(
                context,
                &self.text,
                DrawParam::default().dest(Point2::new(
                    self.conf.window_mode.width * 0.11,
                    2.6 * self.conf.window_mode.height / 4.0,
                )),
            )?;
        }

        Ok(())
    }
}
