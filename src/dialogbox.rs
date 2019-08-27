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
    display: Option<(Dialog, DialogTree, usize)>,
    font: Font,
    mesh: Mesh,
    conf: Conf,
}

impl DialogBox {
    pub fn new(context: &mut Context) -> DialogBox {
        let conf = Conf::new();

        DialogBox {
            display: None,
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
        self.display.is_some()
    }

    pub fn update(&mut self) {}

    pub fn draw(&self, context: &mut Context) -> GameResult {
        if let Some((dialog, _, selected_response)) = &self.display {
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

            for (i, response) in dialog.responses.iter().enumerate() {
                let color = if &i == selected_response {
                    constants::GOLD
                } else {
                    constants::WHITE
                };

                let text = Text::new(
                    TextFragment::new(response.1.as_str())
                        .font(self.font)
                        .scale(Scale::uniform(40.0)),
                );

                graphics::draw(
                    context,
                    &text,
                    DrawParam::default()
                        .dest(Point2::new(
                            self.conf.window_mode.width * 0.11,
                            (2.6 + (0.25 * (i + 1) as f32)) * self.conf.window_mode.height / 4.0,
                        ))
                        .color(color),
                )?;
            }
        }

        Ok(())
    }

    pub fn populate_display(&mut self, dialogtree: Option<DialogTree>) {
        if let Some(dialogtree) = &dialogtree {
            let dialog = dialogtree.dialogs.get(&0).unwrap();
            self.display = Some((dialog.clone(), dialogtree.clone(), 0));
        } else {
            self.display = None;
        }
    }

    pub fn choose_reponse(&mut self) {
        if let Some((dialog, dialogtree, selected_response)) = &self.display.clone() {
            if let Some(selected_dialog) = dialog.responses.get(*selected_response) {
                if let Some(new_dialog) = dialogtree.dialogs.get(&selected_dialog.0) {
                    self.display = Some((new_dialog.clone(), dialogtree.clone(), 0));
                }
            }
        }
    }

    pub fn next_response(&mut self) {
        if let Some((dialog, dialogtree, selected_response)) = &self.display.clone() {
            let new_selected_response =
                if Some(*selected_response) < dialog.responses.len().checked_sub(1) {
                    selected_response + 1
                } else {
                    0
                };

            self.display = Some((dialog.clone(), dialogtree.clone(), new_selected_response));
        }
    }

    pub fn prev_response(&mut self) {
        if let Some((dialog, dialogtree, selected_response)) = &self.display.clone() {
            let new_selected_response = if selected_response == &0 {
                match dialog.responses.len().checked_sub(1) {
                    Some(i) => i,
                    None => 0,
                }
            } else {
                selected_response - 1
            };

            self.display = Some((dialog.clone(), dialogtree.clone(), new_selected_response));
        }
    }
}
