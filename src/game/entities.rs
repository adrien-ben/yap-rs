use crate::game::input::*;
use graphics::{Context, ellipse, Graphics, rectangle, Transformed};
use crate::math::*;
use num;
use rand::random;

const WHITE: [f32; 4] = [1.0; 4];
const MAX_BALL_SPEED: f64 = 1200.0;

pub trait Physics<T> {
    fn world_space_bound(&self) -> T;
}

#[derive(Copy, Clone)]
pub struct Ball {
    pub position: Vector,
    pub direction: Vector,
    pub speed: f64,
    pub size: f64,
}

impl Ball {
    pub fn new(position: Vector, speed: f64, size: f64) -> Self {
        Ball {
            position,
            direction: Self::random_dir(),
            speed,
            size,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.position += self.direction * self.speed * dt;
    }

    pub fn on_hit(&mut self) {
        self.direction.y *= -1.0;
        self.speed = num::clamp(self.speed * 1.1, 0.0, MAX_BALL_SPEED);
    }

    pub fn reset(&mut self, position: Vector, speed: f64) {
        self.position = position;
        self.direction = Self::random_dir();
        self.speed = speed;
    }

    fn random_dir() -> Vector {
        let dir = Vector {
            x: random::<f64>() * 2.0 - 1.0,
            y: random::<f64>() * 10.0 - 5.0,
        };
        dir.normalize()
    }

    pub fn draw<G: Graphics>(&self, area: Rectangle, context: &Context, graphics: &mut G) {
        ellipse(
            WHITE,
            [-self.size * 0.5, -self.size * 0.5, self.size, self.size],
            context.transform.trans(self.position.x, area.top_right.y - self.position.y),
            graphics,
        );
    }
}

impl Physics<Circle> for Ball {
    fn world_space_bound(&self) -> Circle {
        Circle {
            center: self.position,
            radius: self.size * 0.5,
        }
    }
}

pub struct Paddle {
    pub position: Vector,
    pub bound: Rectangle,
    pub speed: f64,
}

impl Paddle {
    pub fn new(position: Vector, bound: Rectangle) -> Self {
        Self {
            position,
            bound,
            speed: 220.0,
        }
    }

    pub fn update(&mut self, player_input: Input, dt: f64) {
        match player_input {
            Input {
                left: true,
                right: false,
                ..
            } => self.move_left(dt),
            Input {
                left: false,
                right: true,
                ..
            } => self.move_right(dt),
            _ => {}
        }
    }

    fn move_right(&mut self, dt: f64) {
        self.position.x += self.speed * dt;
    }

    fn move_left(&mut self, dt: f64) {
        self.position.x -= self.speed * dt;
    }

    pub fn draw<G: Graphics>(&self, area: Rectangle, context: &Context, graphics: &mut G) {
        rectangle(
            WHITE,
            [
                self.bound.bottom_left.x,
                self.bound.bottom_left.y,
                self.bound.width(),
                self.bound.height(),
            ],
            context.transform.trans(self.position.x, area.height() - self.position.y),
            graphics,
        );
    }
}

impl Physics<Rectangle> for Paddle {
    fn world_space_bound(&self) -> Rectangle {
        Rectangle::new(
            self.position + self.bound.bottom_left,
            self.position + self.bound.top_right,
        )
    }
}

pub struct Player {
    pub paddle: Paddle,
    pub input: Input,
    pub score: u32,
}

impl Player {
    pub fn new(paddle: Paddle, input: Input) -> Self {
        Player {
            paddle,
            input,
            score: 0,
        }
    }

    pub fn inc_score(&mut self) {
        self.score += 1;
    }
}