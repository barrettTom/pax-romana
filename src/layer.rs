use ggez::graphics::spritebatch::SpriteBatch;

use crate::entity::Operable;
use crate::tile::Tile;
use crate::tileset::Tileset;

#[derive(Debug, Clone)]
pub struct Layer {
    pub tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Layer {
    pub fn new(text: &str, tileset: &Tileset, width: usize, height: usize) -> Layer {
        Layer {
            tiles: text
                .replace("\n", "")
                .split(',')
                .enumerate()
                .map(|(i, s)| Tile::new(s, i, tileset, width, height))
                .collect(),
            width,
            height,
        }
    }

    pub fn update(&mut self) {
        for tile in self.tiles.iter_mut() {
            tile.update();
        }
    }

    pub fn draw(&self, spritebatch: &mut SpriteBatch) {
        for tile in self.tiles.iter() {
            tile.draw(spritebatch);
        }
    }
}
