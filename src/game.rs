use std::ops::{Add, Mul};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

use crate::config::{HEIGHT, PIPE_A_COLOR, PIPE_B_COLOR, PIPE_GAP_BOUND, PIPE_GAP_SIZE, PIPE_G_COLOR, PIPE_R_COLOR, PIPE_SPACING, PIPE_SPEED, PIPE_WIDTH, SEED, WIDTH};

pub struct Game {
    _score: u32,
    pipes: Vec<Pipe>,
    rng: Pcg32,
}

impl Game {
    pub fn new() -> Self {
        let mut rng = Pcg32::seed_from_u64(SEED);
        let mut pipes = Vec::new();
        // Create first pipe to start creation loop.
        pipes.push(Pipe::new(&mut rng));

        Self { 
            _score: 0,
            pipes,
            rng,
        }
    }

    pub fn update(&mut self, dt: f32) -> () {
       self.check_for_new_pipe();

        for pipe in &mut self.pipes {
            pipe.update(dt);
        }
    }

    pub fn draw(&self, frame: &mut [u8]) -> () {
        for pipe in &self.pipes {
            pipe.draw(frame);
        }
    }

    fn check_for_new_pipe(&mut self) -> () {
        let last_pipe = self.pipes.last().unwrap();

        if last_pipe.position.x as u32 + PIPE_WIDTH + PIPE_SPACING < WIDTH {
            self.pipes.push(Pipe::new(&mut self.rng));
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
    y_gap_location: u32,
}

impl Pipe {
    /// On creation, pipes are placed just past the right side of the screen.
    fn new(rng: &mut Pcg32) -> Self {
        Self {
            position: Vector2D::new(WIDTH as f32, 0.0),
            velocity: Vector2D::new(-PIPE_SPEED, 0.0),
            y_gap_location: rng.random_range(PIPE_GAP_BOUND..HEIGHT - PIPE_GAP_BOUND - PIPE_GAP_SIZE),
        }
    }

    fn update(&mut self, dt: f32) -> () {
        self.position = self.position + self.velocity * dt;
    }

    fn draw(&self, frame: &mut [u8]) -> () {
        let stride = WIDTH as usize * 4;
        let x_position = self.position.x as i32;
        
        for x in x_position..x_position + PIPE_WIDTH as i32 {
            // Don't draw out of bounds
            if x < 0 || x * 4 >= stride as i32 {
                continue;
            }

            for y in 0..HEIGHT as usize {
                if y > self.y_gap_location as usize && y < self.y_gap_location as usize + PIPE_GAP_SIZE as usize {
                    continue;
                }

                let idx = y * stride + x as usize * 4;

                frame[idx + 0] = PIPE_R_COLOR;
                frame[idx + 1] = PIPE_G_COLOR;
                frame[idx + 2] = PIPE_B_COLOR;
                frame[idx + 3] = PIPE_A_COLOR;
            }
        }
    }
}