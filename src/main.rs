use ggez::conf::{NumSamples, WindowSetup};
use ggez::{event, ContextBuilder, GameResult};

use pax_romana::game::Game;

fn main() -> GameResult {
    let (ref mut context, ref mut event_loop) = ContextBuilder::new("pax-romana", "tom barrett")
        .window_setup(
            WindowSetup::default()
                .title("pax_romana")
                .samples(NumSamples::Two),
        )
        .add_resource_path("./resources")
        .build()?;

    let game = &mut Game::new(context)?;

    event::run(context, event_loop, game)
}
