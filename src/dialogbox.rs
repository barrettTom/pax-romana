use ggez::conf::Conf;
use ggez::graphics::{
    self, DrawMode, DrawParam, Font, Mesh, MeshBuilder, Rect, Scale, Text, TextFragment,
};
use ggez::nalgebra::Point2;
use ggez::{filesystem, Context, GameResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::constants;
use crate::npc::Character;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dialog {
    text: String,
    responses: Vec<(usize, String)>,
}

#[derive(Clone, Debug, Default)]
pub struct DialogTree {
    dialogs: HashMap<usize, Dialog>,
}

impl DialogTree {
    pub fn new(context: &mut Context, character: Character) -> DialogTree {
        DialogTree {
            dialogs: serde_json::from_reader(
                filesystem::open(context, "/dialogtrees/".to_string() + character.to_str())
                    .unwrap(),
            )
            .unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct DialogBox {
    dialogtree: Option<DialogTree>,
    dialog: Option<Dialog>,
    font: Font,
    mesh: Mesh,
    conf: Conf,
}

impl DialogBox {
    pub fn new(context: &mut Context) -> DialogBox {
        let conf = Conf::new();

        DialogBox {
            dialogtree: None,
            dialog: None,
            font: Font::new(context, "/fonts/SONORM__.ttf").unwrap(),
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
            conf,
        }
    }

    pub fn is_visible(&self) -> bool {
        self.dialogtree.is_some()
    }

    pub fn update(&mut self) {
        if self.dialogtree.is_none() {
            self.dialog = None;
        }
    }

    pub fn draw(&self, context: &mut Context) -> GameResult {
        if let Some(dialog) = &self.dialog {
            let text = Text::new(
                TextFragment::new(dialog.text.as_str())
                    .font(self.font)
                    .scale(Scale::uniform(40.0)),
            );

            graphics::draw(context, &self.mesh, DrawParam::default())?;
            graphics::draw(
                context,
                &text,
                DrawParam::default().dest(Point2::new(
                    self.conf.window_mode.width * 0.11,
                    2.6 * self.conf.window_mode.height / 4.0,
                )),
            )?;
        }

        Ok(())
    }

    pub fn give_dialogtree(&mut self, dialogtree: Option<DialogTree>) {
        self.dialogtree = dialogtree;
        if let Some(dialogtree) = &self.dialogtree {
            self.dialog = dialogtree.dialogs.get(&0).cloned();
        }
    }
}
