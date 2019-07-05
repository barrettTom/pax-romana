use ggez::graphics::spritebatch::SpriteBatch;

use crate::entity::Operable;
use crate::tile::Cell;
use crate::tileset::Tileset;

#[derive(Debug, Clone)]
pub struct Layer {
    pub cells: Vec<Cell>,
    dimensions: (usize, usize),
}

impl Operable for Layer {
    fn update(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.update();
        }
    }

    fn draw(&self, spritebatch: &mut SpriteBatch) {
        for cell in self.cells.iter() {
            cell.draw(spritebatch);
        }
    }
}

impl Layer {
    pub fn new(text: &str, tileset: &Tileset, dimensions: (usize, usize)) -> Layer {
        Layer {
            cells: text
                .replace("\n", "")
                .split(',')
                .enumerate()
                .map(|(i, s)| Cell::new(s, i, tileset, dimensions))
                .collect(),
            dimensions,
        }
    }
}
