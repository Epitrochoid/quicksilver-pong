extern crate quicksilver;

use quicksilver::{
    State, run,
    geom::{Circle, Rectangle, Transform, Vector},
    graphics::{Color, Draw, Window, WindowBuilder},
    input::{Key, ButtonState}
};

const PADDLE_HEIGHT: f32 = 250.0;
const PADDLE_WIDTH: f32 = 25.0;
const BALL_RADIUS: f32 = 20.0;

const SCREEN_WIDTH: f32 = 1800.0;
const SCREEN_HEIGHT: f32 = 1200.0;

struct DrawGeometry {
    p1_pos: Vector,
    p1_vel: f32,

    p2_pos: Vector,
    p2_vel: f32,

    ball_pos: Vector,
    ball_vel: Vector
}

impl State for DrawGeometry {
    fn new() -> DrawGeometry {
        DrawGeometry {
            p1_pos: Vector::new(0.0, SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0),
            p1_vel: 0.0,

            p2_pos: Vector::new((SCREEN_WIDTH - PADDLE_WIDTH), SCREEN_HEIGHT / 2.0),
            p2_vel: 0.0,

            ball_pos: Vector::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            ball_vel: Vector::new(0.0, 0.0)
        }
    }

    fn draw(&mut self, window: &mut Window) {
        let paddle1: Rectangle = Rectangle::new(self.p1_pos.x, self.p1_pos.y, PADDLE_WIDTH, PADDLE_HEIGHT);

        window.clear(Color::black());
        window.draw(&Draw::rectangle(paddle1).with_color(Color::blue()));
        window.present();
    }
}

fn main() {
    run::<DrawGeometry>(WindowBuilder::new("Quicksilver Pong", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32));
}
