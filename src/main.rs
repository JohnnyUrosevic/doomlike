use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

const SCREEN_WIDTH : f32 = 1280.0;
const SCREEN_HEIGHT : f32 = 720.0;

struct GameState {
    pos_x: f32,
}

impl GameState {
    fn new() -> GameResult<GameState> {
        let s = GameState { pos_x: 0.0 };
        Ok(s)
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(0.0, 0.0),
            100.0,
            2.0,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, (na::Point2::new(self.pos_x, 380.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("doom", "ggez")
    .window_setup(ggez::conf::WindowSetup::default().title("Shooter"))
    .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GameState::new()?;
    event::run(ctx, event_loop, state)
}
