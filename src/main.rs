use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

const SCREEN_WIDTH : u16 = 1280;
const SCREEN_HEIGHT : u16 = 720;
const NUM_PIXELS : usize = SCREEN_HEIGHT as usize * SCREEN_WIDTH as usize * 4;

const MAP_WIDTH : usize = 24;
const MAP_HEIGHT : usize = 24;


struct GameState {
    pos: na::Point2<f32>,
    dir: f32,
    map: [[u8; MAP_WIDTH]; MAP_HEIGHT],
}

impl GameState {
    fn new() -> GameResult<GameState> {
        let map = [
            [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
            [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
            [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1],
            [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,2,2,0,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
            [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
        ];
        let s = GameState { 
            pos: na::Point2::new(20.0, 12.0),
            dir: 0.0,
            map,
        };
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

        let view_left = na::Vector2::new((self.dir + 0.1).cos(), (self.dir + 0.1).sin());
        let view_right = na::Vector2::new((self.dir - 0.1).cos(), (self.dir - 0.1).sin());

        for x in 0..SCREEN_WIDTH {
            let camera_x = x / SCREEN_WIDTH;
            let ray_dir = na::Vector2::new(view_left.x * (1 - camera_x) as f32, view_left.y * (1 - camera_x) as f32) 
                + na::Vector2::new(view_right.x * camera_x as f32, view_right.y * camera_x as f32);

            let skip = na::Vector2::new(1.0 / ray_dir.x.abs(), 1.0 / ray_dir.y.abs());

            let dist_x = if ray_dir.x > 0.0 {
                self.pos.x.ceil() - self.pos.x
            }
            else {
                self.pos.x - self.pos.x.floor()
            };

            let dist_y = if ray_dir.y > 0.0 {
                self.pos.y.ceil() - self.pos.y
            }
            else {
                self.pos.y - self.pos.y.floor()
            };

            let mut dist = na::Vector2::new(dist_x * skip.x, dist_y * skip.y);

            let mut map = na::Vector2::new(self.pos.x as isize, self.pos.y as isize);
            let mut traveled = 0.0;

            let mut hit = false;
            while !hit {
                if dist.x < dist.y {
                    map.x += dist.x.signum() as isize;
                    traveled += dist.x;
                    dist.y -= dist.x;
                    dist.x = skip.x;
                }
                else {
                    map.y += dist.y.signum() as isize;
                    traveled += dist.y;
                    dist.x -= dist.y;
                    dist.y = skip.y;
                }

                println!("{}, {:?} {:?}", x, map.x, map.y);
                if self.map[map.y as usize][map.x as usize] > 0 {
                    hit = true;
                }
            }
        }

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
