use ggez::filesystem::File;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::nalgebra::Point2;
use std::collections::HashMap;
use xml::reader::XmlEvent::Characters;

use crate::constants;
use crate::entity::Operable;
use crate::layer::Layer;
use crate::tileset::{Tile, Tileset};
use crate::xmlelements::XMLElements;

#[derive(Clone)]
pub struct Map {
    dimensions: (usize, usize),
    layers: Vec<Layer>,
    spawns: Vec<(String, Point2<f32>)>,
}

impl Operable for Map {
    fn draw(&self, spritebatch: &mut SpriteBatch) {
        for layer in self.layers.iter() {
            layer.draw(spritebatch);
        }
    }

    fn update(&mut self) {
        for layer in self.layers.iter_mut() {
            layer.update();
        }
    }
}

impl Map {
    pub fn new(file: File, tileset: &Tileset) -> Map {
        let elements = XMLElements::new(file);

        let dimensions = (
            elements
                .get_element_attribute("map", "width")
                .unwrap()
                .parse()
                .unwrap(),
            elements
                .get_element_attribute("map", "height")
                .unwrap()
                .parse()
                .unwrap(),
        );

        let layers: Vec<Layer> = elements
            .events
            .iter()
            .filter_map(|e| {
                if let Characters(text) = e {
                    Some(Layer::new(text, tileset, dimensions))
                } else {
                    None
                }
            })
            .collect();

        let spawns = Map::get_spawn_points(&layers, tileset.get_spawn_tiles());

        Map {
            layers,
            dimensions,
            spawns,
        }
    }

    fn get_spawn_points(
        layers: &[Layer],
        spawn_tiles: HashMap<usize, Tile>,
    ) -> Vec<(String, Point2<f32>)> {
        let mut spawn_points = Vec::new();

        for layer in layers.iter() {
            for cell in layer.cells.iter() {
                for (id, tile) in spawn_tiles.iter() {
                    if id == &cell.id {
                        spawn_points.push((tile.property.spawn.clone().unwrap(), cell.destination));
                    }
                }
            }
        }

        spawn_points
    }

    pub fn get_spawns(&self) -> Vec<(String, Point2<f32>)> {
        self.spawns.clone()
    }

    pub fn get_dimensions(&self) -> (f32, f32) {
        (
            (constants::TILE_WIDTH * constants::TILE_SCALE) * self.dimensions.0 as f32,
            (constants::TILE_HEIGHT * constants::TILE_SCALE) * self.dimensions.1 as f32,
        )
    }
}
