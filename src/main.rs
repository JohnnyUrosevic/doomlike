use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

const SCREEN_WIDTH : u16 = 1280;
const SCREEN_HEIGHT : u16 = 720;
const NUM_PIXELS : usize = SCREEN_HEIGHT as usize * SCREEN_WIDTH as usize * 4;

struct GameState {
    pos: na::Vector2<f32>,
}

impl GameState {
    fn new() -> GameResult<GameState> {
        let s = GameState { pos: na::Vector2::new(1.0, 0.0) };
        Ok(s)
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let sky_blue : [u8; 4] = [135, 206, 235, 255];
        let grey : [u8; 4] = [128, 128, 128, 255];
        let mut frame : Vec<u8> = sky_blue.iter().cloned().cycle().take(NUM_PIXELS).collect();

        let mut draw = |x: usize, y: usize, color: [u8; 4]| {
            let index = (y * SCREEN_WIDTH as usize + x) * 4;
            frame[index..(4 + index)].clone_from_slice(&color);
        };

        draw(SCREEN_WIDTH as usize / 2, SCREEN_HEIGHT as usize / 2, grey);

        let image = graphics::Image::from_rgba8(ctx, SCREEN_WIDTH, SCREEN_HEIGHT, frame.as_slice())?;
        graphics::draw(ctx, &image, (na::Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("doom", "ggez")
    .window_setup(ggez::conf::WindowSetup::default().title("Shooter"))
    .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GameState::new()?;
    event::run(ctx, event_loop, state)
}
