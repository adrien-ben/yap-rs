use crate::math::{Rectangle, Vector};
use piston::input::Key;
use specs::prelude::*;
use specs_derive::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Position {
    pub current: Vector,
    pub default: Vector,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Position {
            current: Vector::new(x, y),
            default: Vector::new(x, y),
        }
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Velocity {
    pub direction: Vector,
    pub speed: f64,
    pub max_speed: f64,
}

impl Velocity {
    pub fn new(direction: Vector, max_speed: f64) -> Self {
        Velocity {
            direction,
            speed: max_speed,
            max_speed,
        }
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Ball {
    pub size: f64,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Paddle {
    pub bound: Rectangle,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Collision;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Input {
    pub key_right: Key,
    pub key_left: Key,
    pub right: bool,
    pub left: bool,
}

impl Input {
    pub fn new(key_left: Key, key_right: Key) -> Input {
        Input {
            key_right,
            key_left,
            right: false,
            left: false,
        }
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Score {
    pub current: u32,
    pub trigger: fn(Vector) -> bool,
    pub position: Vector,
}

impl Score {
    pub fn new(trigger: fn(Vector) -> bool, position: Vector) -> Score {
        Score {
            current: 0,
            trigger,
            position,
        }
    }
}
