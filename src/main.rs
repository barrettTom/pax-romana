use ggez::{conf::Conf, event, ContextBuilder, GameResult};

use pax_romana::state::State;

fn main() -> GameResult {
    let conf = Conf::new();

    let (ref mut context, ref mut event_loop) = ContextBuilder::new("pax-romana", "tom barrett")
        .conf(conf)
        .add_resource_path("./resources")
        .build()?;

    let state = &mut State::new(context)?;

    event::run(context, event_loop, state)
}
