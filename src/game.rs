use ggez;
use ggez::event;
use ggez::event::KeyCode;
use ggez::graphics;
use ggez::{Context, GameResult};

use crate::states;
use crate::states::StatesResult;

pub struct Game {
    states: Vec<Box<dyn states::States>>,
}

impl Game {
    pub fn new(_ctx: &mut Context) -> GameResult<Game> {
        // 초기에는 InitState를 넣는다.

        let init_state = states::InitStates::new();

        let s = Game {
            states: vec![Box::new(init_state)],
        };
        Ok(s)
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // 닫기가 눌러지면 게임 종료한다.
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Escape) {
            ggez::event::quit(ctx);
        }

        // 현재 states 값을 얻어와 해당 states의 update 를 실행한다.
        let current_state = self.states.last_mut().unwrap();

        match current_state.update(ctx) {
            StatesResult::PushState(s) => self.states.push(s),
            StatesResult::PopState => {
                self.states.pop();
                ()
            }
            _ => (),
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // 현재 states 값을 얻어와 해당 states의 render 를 실행한다.
        let current_state = self.states.last_mut().unwrap();

        current_state.render(ctx);
        Ok(())
    }
}
