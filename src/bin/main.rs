use ggez;
use ggez::event;
use ggez::GameResult;
use ggez_skel;

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Hello World", "siabard");
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut ggez_skel::game::Game::new(ctx)?;

    event::run(ctx, event_loop, state)
}
