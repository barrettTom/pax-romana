use ggez::filesystem::File;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::nalgebra::Point2;
use xml::reader::XmlEvent::Characters;

use crate::constants;
use crate::layer::Layer;
use crate::tileset::Tileset;
use crate::xmlelements::XMLElements;

#[derive(Clone)]
pub struct Map {
    width: usize,
    height: usize,
    layers: Vec<Layer>,
    spawns: Vec<(String, Point2<f32>)>,
}

impl Map {
    pub fn new(file: File, tileset: &Tileset) -> Map {
        let elements = XMLElements::new(file);

        let width = elements
            .get_element_attribute("map", "width")
            .unwrap()
            .parse()
            .unwrap();
        let height = elements
            .get_element_attribute("map", "height")
            .unwrap()
            .parse()
            .unwrap();

        let layers: Vec<Layer> = elements
            .events
            .iter()
            .filter_map(|e| {
                if let Characters(text) = e {
                    Some(Layer::new(text, tileset, width, height))
                } else {
                    None
                }
            })
            .collect();

        let spawns = Map::get_spawn_points(&layers, tileset.get_spawn_tiles());

        Map {
            layers,
            width,
            height,
            spawns,
        }
    }

    fn get_spawn_points(
        layers: &[Layer],
        spawn_tiles: Vec<(String, usize)>,
    ) -> Vec<(String, Point2<f32>)> {
        let mut spawn_points = Vec::new();

        for layer in layers.iter() {
            for tile in layer.tiles.iter() {
                for spawn_tile in spawn_tiles.iter() {
                    if spawn_tile.1 == tile.id {
                        spawn_points.push((spawn_tile.0.clone(), tile.destination));
                    }
                }
            }
        }

        spawn_points
    }

    pub fn draw(&self, spritebatch: &mut SpriteBatch) {
        for layer in self.layers.iter() {
            layer.draw(spritebatch);
        }
    }

    pub fn update(&mut self) {
        for layer in self.layers.iter_mut() {
            layer.update();
        }
    }

    pub fn get_spawns(&self) -> Vec<(String, Point2<f32>)> {
        self.spawns.clone()
    }

    pub fn get_dimensions(&self) -> (f32, f32) {
        (
            (constants::TILE_WIDTH * constants::TILE_SCALE) * self.width as f32,
            (constants::TILE_HEIGHT * constants::TILE_SCALE) * self.height as f32,
        )
    }
}
