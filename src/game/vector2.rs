use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn up() -> Self {
        Self { x: 0.0, y: -1.0 }
    }

    pub fn down() -> Self {
        Self { x: 0.0, y: 1.0 }
    }

    pub fn left() -> Self {
        Self { x: -1.0, y: 0.0 }
    }
}

impl Add for Vector2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;
    fn mul(self, s: f32) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
        }
    }
}
