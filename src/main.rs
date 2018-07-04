extern crate quicksilver;
extern crate nalgebra as na;
extern crate ncollide2d;
extern crate rand;

use na::{Isometry2, Vector2};
use ncollide2d::shape::{Ball, Cuboid};

use quicksilver::{
    State, run,
    geom::{Circle, Rectangle, Transform, Vector},
    graphics::{Color, Draw, Window, WindowBuilder},
    input::{Key, ButtonState}
};

const PADDLE_HEIGHT: f32 = 250.0;
const PADDLE_WIDTH: f32 = 25.0;
const PADDLE_VELOCITY: f32 = 8.0;
const BALL_RADIUS: f32 = 20.0;

const SCREEN_WIDTH: f32 = 1800.0;
const SCREEN_HEIGHT: f32 = 1200.0;

struct DrawGeometry {
    p1_pos: Vector,
    p1_vel: f32,
    p1_bbox: Cuboid<f32>,

    p2_pos: Vector,
    p2_vel: f32,
    p2_bbox: Cuboid<f32>,

    ball_pos: Vector,
    ball_dir: Vector,
    ball_speed: f32,
    ball_bbox: Ball<f32>
}

impl State for DrawGeometry {
    fn new() -> DrawGeometry {
        DrawGeometry {
            p1_pos: Vector::new(0.0, SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0),
            p1_vel: 0.0,
            p1_bbox: Cuboid::new(Vector2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),

            //p2_pos: Vector::new(SCREEN_WIDTH - PADDLE_WIDTH, SCREEN_HEIGHT / 2.0),
            p2_pos: Vector::new(SCREEN_WIDTH - PADDLE_WIDTH, SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0),
            p2_vel: 0.0,
            p2_bbox: Cuboid::new(Vector2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),

            ball_pos: Vector::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            ball_dir: Vector::new(2.0 * rand::random::<f32>(), rand::random::<f32>()).normalize(),
            ball_speed: 6.0,
            ball_bbox: Ball::new(BALL_RADIUS)
        }
    }

    fn update(&mut self, window: &mut Window) {
        // Physics
        fn calc_position_change(y_pos: f32, y_vel: f32) -> Option<f32> {
            let new_pos = y_pos + y_vel;

            if (new_pos > 0.0 && ((new_pos + PADDLE_HEIGHT) < SCREEN_HEIGHT)) {
                return Some(new_pos);
            } else {
                return None;
            }
        }

        // Player 1
        if let Some(new_pos) = calc_position_change(self.p1_pos.y, self.p1_vel) {
            self.p1_pos.y = new_pos;
        }

        // Player 2
        if let Some(new_pos) = calc_position_change(self.p2_pos.y, self.p2_vel) {
            self.p2_pos.y = new_pos;
        }

        // Ball
        let mut new_ball_pos = (self.ball_dir * self.ball_speed) + self.ball_pos;

        if (new_ball_pos.y < 0.0 || new_ball_pos.y > SCREEN_HEIGHT) {
           self.ball_dir.y = -self.ball_dir.y;
           new_ball_pos = (self.ball_dir * self.ball_speed) + self.ball_pos;
        }

        self.ball_pos = new_ball_pos;

        // Input
        // Player 1
        if window.keyboard()[Key::Q] == ButtonState::Held {
            self.p1_vel = -PADDLE_VELOCITY;
        } else if window.keyboard()[Key::A] == ButtonState::Held {
            self.p1_vel = PADDLE_VELOCITY;
        } else {
            self.p1_vel = 0.0;
        }

        // Player 2
        if window.keyboard()[Key::P] == ButtonState::Held {
            self.p2_vel = -PADDLE_VELOCITY;
        } else if window.keyboard()[Key::L] == ButtonState::Held {
            self.p2_vel = PADDLE_VELOCITY;
        } else {
            self.p2_vel = 0.0;
        }
    }

    fn draw(&mut self, window: &mut Window) {
        let paddle1: Rectangle = Rectangle::new(self.p1_pos.x, self.p1_pos.y, PADDLE_WIDTH, PADDLE_HEIGHT);
        let paddle2: Rectangle = Rectangle::new(self.p2_pos.x, self.p2_pos.y, PADDLE_WIDTH, PADDLE_HEIGHT);

        let ball: Circle = Circle::new(self.ball_pos.x, self.ball_pos.y, BALL_RADIUS);

        window.clear(Color::black());
        window.draw(&Draw::rectangle(paddle1).with_color(Color::blue()));
        window.draw(&Draw::rectangle(paddle2).with_color(Color::blue()));
        window.draw(&Draw::circle(ball).with_color(Color::red()));
        window.present();
    }
}

fn main() {
    run::<DrawGeometry>(WindowBuilder::new("Quicksilver Pong", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32));
}
