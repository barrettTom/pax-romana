use ggez::conf::Conf;
use ggez::event::{self, EventHandler};
use ggez::filesystem;
use ggez::graphics::{self, spritebatch::SpriteBatch, DrawParam, FilterMode, Image};
use ggez::nalgebra::{Point2, Vector2};
use ggez::timer::delta;
use ggez::{Context, ContextBuilder, GameResult};
use std::collections::HashMap;
use std::io::Read;
use std::time::Duration;

struct State {
    dt: Duration,
    spritebatches: HashMap<Tiles, SpriteBatch>,
    map: Vec<char>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Tiles {
    Grass,
    Dirt,
}

impl State {
    fn new(context: &mut Context) -> GameResult<State> {
        let mut grass = Image::new(context, "/grass.png")?;
        grass.set_filter(FilterMode::Nearest);

        let mut dirt = Image::new(context, "/dirt.png")?;
        dirt.set_filter(FilterMode::Nearest);

        let mut spritebatches = HashMap::new();
        spritebatches.insert(Tiles::Grass, SpriteBatch::new(grass));
        spritebatches.insert(Tiles::Dirt, SpriteBatch::new(dirt));

        let mut map_str = String::new();
        filesystem::open(context, "/map.txt")
            .unwrap()
            .read_to_string(&mut map_str)
            .unwrap();

        Ok(State {
            dt: Duration::new(0, 0),
            spritebatches,
            map: map_str.replace("\n", "").chars().collect(),
        })
    }
}

const TILE_SIZE: f32 = 16.0;
const TILE_SCALE: f32 = 4.0;

impl EventHandler for State {
    fn update(&mut self, context: &mut Context) -> GameResult {
        self.dt = delta(context);
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, graphics::BLACK);

        for x in 0..4 {
            for y in 0..4 {
                let draw_param = DrawParam::default()
                    .dest(Point2::new(
                        TILE_SIZE * TILE_SCALE * x as f32,
                        TILE_SIZE * TILE_SCALE * y as f32,
                    ))
                    .scale(Vector2::new(TILE_SCALE, TILE_SCALE));

                let index = (x as usize) + (y as usize * 4);
                match self.map[index] {
                    'G' => {
                        self.spritebatches
                            .get_mut(&Tiles::Grass)
                            .unwrap()
                            .add(draw_param);
                    }
                    'D' => {
                        self.spritebatches
                            .get_mut(&Tiles::Dirt)
                            .unwrap()
                            .add(draw_param);
                    }
                    _ => (),
                }
            }
        }

        for (_, spritebatch) in self.spritebatches.iter_mut() {
            graphics::draw(context, spritebatch, DrawParam::default())?;
            spritebatch.clear();
        }

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
