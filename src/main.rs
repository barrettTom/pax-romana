use ggez::conf::Conf;
use ggez::event::{self, EventHandler};
use ggez::filesystem;
use ggez::graphics::{self, spritebatch::SpriteBatch, DrawParam, FilterMode, Image, Rect};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, ContextBuilder, GameResult};
use std::io::BufReader;
use tiled::{parse, Map};

struct State {
    spritebatch: SpriteBatch,
    map: Map,
}

impl State {
    fn new(context: &mut Context) -> GameResult<State> {
        let mut tileset = Image::new(context, "/tileset.png")?;
        tileset.set_filter(FilterMode::Nearest);

        let reader = BufReader::new(filesystem::open(context, "/map.tmx")?);

        Ok(State {
            spritebatch: SpriteBatch::new(tileset),
            map: parse(reader).unwrap(),
        })
    }
}

const TILE_SIZE: f32 = 16.0;
const TILE_SCALE: f32 = 3.0;

impl EventHandler for State {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, graphics::BLACK);

        for layer in self.map.layers.iter() {
            for x in 0..self.map.width {
                for y in 0..self.map.height {
                    let draw_param = DrawParam::default()
                        .dest(Point2::new(
                            TILE_SIZE * TILE_SCALE * x as f32,
                            TILE_SIZE * TILE_SCALE * y as f32,
                        ))
                        .scale(Vector2::new(TILE_SCALE, TILE_SCALE))
                        .src(match layer.tiles[y as usize][x as usize] {
                            1 => Rect::new(0.0, 0.0, 1.0 / 3.0, 1.0),
                            2 => Rect::new(1.0 / 3.0, 0.0, 2.0 / 3.0, 1.0),
                            3 => Rect::new(2.0 / 3.0, 0.0, 1.0, 1.0),
                            _ => Rect::zero(),
                        });

                    self.spritebatch.add(draw_param);
                }
            }
        }

        graphics::draw(context, &self.spritebatch, DrawParam::default())?;
        self.spritebatch.clear();

        graphics::present(context)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let conf = Conf::new();

    let (ref mut context, ref mut event_loop) = ContextBuilder::new("pax-romana", "tom barrett")
        .conf(conf)
        .add_resource_path("./resources")
        .build()?;

    let state = &mut State::new(context)?;

    event::run(context, event_loop, state)
}
