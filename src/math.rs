use num;
use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Vector {x, y}
    }

    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(self) -> Self {
        let length = self.length();
        Vector {
            x: self.x / length,
            y: self.y / length,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        (*self).x += other.x;
        (*self).y += other.y;
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f64) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub bottom_left: Vector,
    pub top_right: Vector,
}

impl Rectangle {
    pub fn new(bottom_left: Vector, top_right: Vector) -> Self {
        if bottom_left > top_right {
            panic!("Bottom left corner must be gte than top right corner")
        }
        Rectangle {
            bottom_left,
            top_right,
        }
    }

    pub fn width(self) -> f64 {
        self.top_right.x - self.bottom_left.x
    }

    pub fn height(self) -> f64 {
        self.top_right.y - self.bottom_left.y
    }

    pub fn center(self) -> Vector {
        Vector {
            x: self.bottom_left.x + self.width() * 0.5,
            y: self.bottom_left.y + self.height() * 0.5,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Circle {
    pub center: Vector,
    pub radius: f64,
}

pub fn check_collision(rectangle: Rectangle, circle: Circle) -> bool {
    let clamp_center_x = num::clamp(
        circle.center.x,
        rectangle.bottom_left.x,
        rectangle.top_right.x,
    );
    let clamp_center_y = num::clamp(
        circle.center.y,
        rectangle.bottom_left.y,
        rectangle.top_right.y,
    );
    let dist_x = clamp_center_x - circle.center.x;
    let dist_y = clamp_center_y - circle.center.y;
    (dist_x * dist_x + dist_y * dist_y) < (circle.radius * circle.radius)
}

#[cfg(test)]
mod collision_tests {
    use super::*;

    #[test]
    fn it_should_collide_when_rectangle_is_inside_circle() {
        let circle = Circle {
            center: Vector { x: 0.0, y: 0.0 },
            radius: 10.0,
        };
        let rectangle = Rectangle::new(Vector { x: -5.0, y: -5.0 }, Vector { x: 5.0, y: 5.0 });
        assert!(check_collision(rectangle, circle));
    }

    #[test]
    fn it_should_collide_when_circle_is_inside_rectangle() {
        let circle = Circle {
            center: Vector { x: 0.0, y: 0.0 },
            radius: 3.0,
        };
        let rectangle = Rectangle::new(Vector { x: -5.0, y: -5.0 }, Vector { x: 5.0, y: 5.0 });
        assert!(check_collision(rectangle, circle));
    }

    #[test]
    fn it_should_collide_when_overlapping() {
        let circle = Circle {
            center: Vector { x: 0.0, y: 0.0 },
            radius: 3.0,
        };
        let rectangle = Rectangle::new(Vector { x: 2.95, y: -5.0 }, Vector { x: 5.0, y: 5.0 });
        assert!(check_collision(rectangle, circle));
    }

    #[test]
    fn it_should_not_collide_when_separate() {
        let circle = Circle {
            center: Vector { x: 0.0, y: 0.0 },
            radius: 3.0,
        };
        let rectangle = Rectangle::new(Vector { x: 4.0, y: -5.0 }, Vector { x: 5.0, y: 5.0 });
        assert!(!check_collision(rectangle, circle));
    }
}
