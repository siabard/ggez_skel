use ggez;
use ggez::event;
use ggez::event::KeyCode;
use ggez::graphics::{self, Canvas};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use crate::states;
use crate::states::StatesResult;

pub const WINDOW_WIDTH: f32 = 1024.;
pub const WINDOW_HEIGHT: f32 = 768.;

pub const VIRTUAL_WIDTH: f32 = 2048.;
pub const VIRTUAL_HEIGHT: f32 = 768. * 2.;

pub struct Game {
    states: Vec<Box<dyn states::States>>,
    buffer: ggez::graphics::Canvas,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        // 초기에는 InitState를 넣는다.

        let init_state = states::InitStates::new();

        let buffer = ggez::graphics::Canvas::new(
            ctx,
            VIRTUAL_WIDTH as u16,
            VIRTUAL_HEIGHT as u16,
            ggez::conf::NumSamples::One,
        )
        .unwrap();

        let s = Game {
            states: vec![Box::new(init_state)],
            buffer,
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
        let dest_point = na::Point2::new(0., 0.);

        // 전체 화면을 가상의 크기로 설정한다.
        graphics::set_screen_coordinates(
            ctx,
            graphics::Rect::new(0.0, 0.0, VIRTUAL_WIDTH, VIRTUAL_HEIGHT),
        )
        .unwrap();

        // Canvas에 이미지를 그리도록 변경
        graphics::set_canvas(ctx, Some(&self.buffer));

        // 현재 states 값을 얻어와 해당 states의 render 를 실행한다.
        let current_state = self.states.last_mut().unwrap();

        current_state.render(ctx, &mut self.buffer);

        // 이제 메인 윈도우에 그림
        graphics::set_canvas(ctx, None);

        // canvas buffer를 윈도우에 출력
        graphics::clear(ctx, [0.7, 0.7, 0.7, 0.5].into());
        graphics::draw(
            ctx,
            &self.buffer,
            graphics::DrawParam::new()
                .dest(dest_point)
                .src(graphics::Rect::new(0., 0., 1., 1.)),
        )?;

        graphics::present(ctx)?;

        Ok(())
    }
}
