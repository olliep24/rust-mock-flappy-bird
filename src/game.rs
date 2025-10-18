use std::ops::{Add, Mul};

use crate::config::{HEIGHT, PIPE_A_COLOR, PIPE_B_COLOR, PIPE_G_COLOR, PIPE_R_COLOR, PIPE_SPEED, PIPE_WIDTH, WIDTH};

pub struct Game {
    _score: u32,
    pipes: Vec<Pipe>,
}

impl Game {
    pub fn new() -> Self {
        let mut pipes = Vec::new();
        pipes.push(Pipe::new());

        Self { 
            _score: 0,
            pipes,
        }
    }

    pub fn update(&mut self, dt: f32) -> () {
        for pipe in &mut self.pipes {
            pipe.update(dt);
        }
    }

    pub fn draw(&self, frame: &mut [u8]) -> () {
        for pipe in &self.pipes {
            pipe.draw(frame);
        }
    }
}

#[derive(Clone, Copy)]
struct Vector2D {
    x: f32,
    y: f32,
}

impl Vector2D {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Add for Vector2D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { Self { x: self.x + rhs.x, y: self.y + rhs.y } }
}

impl Mul<f32> for Vector2D {
    type Output = Self;
    fn mul(self, s: f32) -> Self { Self { x: self.x * s, y: self.y * s } }
}

/// Struct for the pipe.
/// Pipes' position is the top left corner of them. 
struct Pipe {
    position: Vector2D,
    velocity: Vector2D,
}

impl Pipe {
    /// On creation, pipes are placed just past the right side of the screen.
    fn new() -> Self {
        Self {
            position: Vector2D::new(WIDTH as f32, 0.0),
            velocity: Vector2D::new(-PIPE_SPEED, 0.0),
        }
    }

    fn update(&mut self, dt: f32) -> () {
        self.position = self.position + self.velocity * dt;
    }

    fn draw(&self, frame: &mut [u8]) -> () {
        // clamp or early-return if offscreen
        // if self.position.x < 0.0 || self.position.x >= WIDTH as f32 { 
        //     return; 
        // }
        let stride = WIDTH as usize * 4;
        let x_position = self.position.x as i32;
        
        for x in x_position..x_position + PIPE_WIDTH as i32 {
            // Don't draw out of bounds
            if x < 0 || x * 4 >= stride as i32 {
                continue;
            }

            for y in 0..HEIGHT as usize {
                let idx = y * stride + x as usize * 4;
                if idx + 3 >= frame.len() { break; } // safety

                frame[idx + 0] = PIPE_R_COLOR;
                frame[idx + 1] = PIPE_G_COLOR;
                frame[idx + 2] = PIPE_B_COLOR;
                frame[idx + 3] = PIPE_A_COLOR;
            }
        }
    }
}