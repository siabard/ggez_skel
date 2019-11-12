use ggez;
use ggez::event;
use ggez::GameResult;
use ggez_skel;

use ggez_skel::game::{WINDOW_HEIGHT, WINDOW_WIDTH};

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Hello World", "siabard");
    let (ctx, event_loop) = &mut cb
        .add_resource_path("./resources")
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .window_setup(ggez::conf::WindowSetup::default().title("Skeleton: ggez"))
        .build()?;

    let state = &mut ggez_skel::game::Game::new(ctx)?;

    event::run(ctx, event_loop, state)
}
