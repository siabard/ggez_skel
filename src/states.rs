use ggez::graphics;
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};

use crate::game::Game;

pub enum StatesResult {
    PushState(Box<dyn States>),
    PopState,
    Void,
}

pub trait States {
    fn update(&mut self, ctx: &mut Context) -> StatesResult;
    fn render(&mut self, ctx: &mut Context) -> StatesResult;
}

#[derive(Clone)]
pub struct InitStates;

impl InitStates {
    pub fn new() -> InitStates {
        InitStates
    }
}
impl States for InitStates {
    fn update(&mut self, ctx: &mut Context) -> StatesResult {
        // E가 눌러지면 게임 시작한다.
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::E) {
            let game_state = GameStates::new();
            StatesResult::PushState(Box::new(game_state))
        } else {
            StatesResult::Void
        }
    }

    fn render(&mut self, ctx: &mut Context) -> StatesResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        graphics::present(ctx);

        StatesResult::Void
    }
}

#[derive(Clone)]
pub struct GameStates;

impl GameStates {
    pub fn new() -> GameStates {
        GameStates
    }
}
impl States for GameStates {
    fn update(&mut self, ctx: &mut Context) -> StatesResult {
        // X가 눌러지면 게임 스테이트 종료.
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::X) {
            StatesResult::PopState
        } else {
            StatesResult::Void
        }
    }

    fn render(&mut self, ctx: &mut Context) -> StatesResult {
        graphics::clear(ctx, [1.0, 0.0, 0.0, 1.0].into());

        graphics::present(ctx);

        StatesResult::Void
    }
}
