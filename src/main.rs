use ggez::conf::Conf;
use ggez::event::{self, EventHandler};
use ggez::filesystem;
use ggez::graphics::{self, spritebatch::SpriteBatch, DrawParam, FilterMode, Image};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, ContextBuilder, GameResult};
use pax_romana::constants;
use pax_romana::map::Map;
use pax_romana::tileset::Tileset;

struct State {
    map: Map,
    tileset: Tileset,
    spritebatch: SpriteBatch,
}

impl State {
    fn new(context: &mut Context) -> GameResult<State> {
        let mut image = Image::new(context, "/tileset.png")?;
        image.set_filter(FilterMode::Nearest);

        Ok(State {
            map: Map::new(filesystem::open(context, "/map.tmx")?),
            tileset: Tileset::new(filesystem::open(context, "/tileset.tsx")?),
            spritebatch: SpriteBatch::new(image),
        })
    }
}

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
                        .src(self.tileset.tiles[layer.data[x + (y * self.map.height)]])
                        .dest(Point2::new(
                            self.tileset.tile_width * constants::TILE_SCALE * x as f32,
                            self.tileset.tile_height * constants::TILE_SCALE * y as f32,
                        ))
                        .scale(Vector2::new(constants::TILE_SCALE, constants::TILE_SCALE));

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
