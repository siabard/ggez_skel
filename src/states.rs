use crate::game::Game;
use ggez::graphics::{self, Canvas, DrawMode, DrawParam, Drawable, Rect};
use ggez::input::keyboard::KeyCode;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

pub enum StatesResult {
    PushState(Box<dyn States>),
    PopState,
    Trans(Box<dyn States>),
    Void,
}

pub trait States {
    fn update(&mut self, ctx: &mut Context) -> StatesResult;
    fn render(&mut self, ctx: &mut Context, buffer: &mut Canvas) -> StatesResult;
}

#[derive(Clone)]
pub struct InitStates {}

impl InitStates {
    pub fn new() -> InitStates {
        InitStates {}
    }
}
impl States for InitStates {
    fn update(&mut self, ctx: &mut Context) -> StatesResult {
        // E가 눌러지면 게임 시작한다.
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::E) {
            let game_state = GameStates::new(ctx);
            StatesResult::Trans(Box::new(game_state))
        } else {
            StatesResult::Void
        }
    }

    /// 모든 Render는 이제 자체에 포함된 buffer에만 그린다.
    fn render(&mut self, ctx: &mut Context, buffer: &mut Canvas) -> StatesResult {
        ggez::graphics::set_canvas(ctx, Some(buffer));

        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        graphics::present(ctx);

        ggez::graphics::set_canvas(ctx, None);
        StatesResult::Void
    }
}

#[derive(Clone)]
pub struct PauseStates {}

impl PauseStates {
    pub fn new() -> PauseStates {
        PauseStates {}
    }
}

impl States for PauseStates {
    fn update(&mut self, ctx: &mut Context) -> StatesResult {
        // X가 눌러지면 스테이트 종료
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Q) {
            StatesResult::PopState
        } else {
            StatesResult::Void
        }
    }

    fn render(&mut self, ctx: &mut Context, buffer: &mut Canvas) -> StatesResult {
        ggez::graphics::set_canvas(ctx, Some(buffer));

        graphics::clear(ctx, [0.0, 1.0, 0.0, 1.0].into());

        graphics::present(ctx);

        ggez::graphics::set_canvas(ctx, None);
        StatesResult::Void
    }
}

#[derive(Clone)]
pub struct GameStates {
    sprite: ggez::graphics::Mesh,
    x: f32,
    y: f32,
}

impl GameStates {
    pub fn new(ctx: &mut Context) -> GameStates {
        GameStates {
            sprite: ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                Rect::new(0., 0., 100., 100.),
                ggez::graphics::WHITE,
            )
            .unwrap(),
            x: 0.,
            y: 0.,
        }
    }
}
impl States for GameStates {
    fn update(&mut self, ctx: &mut Context) -> StatesResult {
        // X가 눌러지면 게임 스테이트 종료.
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::X) {
            StatesResult::PopState
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::P) {
            let pause_state = PauseStates::new();
            StatesResult::PushState(Box::new(pause_state))
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.y = self.y - 100.;
            StatesResult::Void
        } else if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.y = self.y + 100.;
            StatesResult::Void
        } else {
            StatesResult::Void
        }
    }

    fn render(&mut self, ctx: &mut Context, buffer: &mut Canvas) -> StatesResult {
        graphics::set_canvas(ctx, Some(buffer));

        graphics::clear(ctx, [1.0, 0.0, 0.0, 1.0].into());

        let dest = na::Point2::new(self.x, self.y);
        self.sprite
            .draw(ctx, DrawParam::default().dest(dest))
            .unwrap();
        graphics::present(ctx);

        graphics::set_canvas(ctx, None);

        StatesResult::Void
    }
}
