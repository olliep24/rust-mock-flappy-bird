use crate::config::{BIRD_FLY_SPEED, BIRD_GRAVITY_ACCELERATION_SCALE, BIRD_HEIGHT, BIRD_R_COLOR, BIRD_A_COLOR, BIRD_B_COLOR, BIRD_G_COLOR, BIRD_START_POSITION_X, BIRD_START_POSITION_Y, BIRD_WIDTH, WIDTH, PIPE_WIDTH};
use crate::game::pipe::Pipe;
use crate::game::vector2::Vector2;

/// Struct for the pipe.
/// A bird's position represents the top left corner of them.
pub struct Bird {
    position: Vector2,
    velocity: Vector2,
}

impl Bird {
    pub fn new() -> Self {
        Self {
            position: Vector2::new(BIRD_START_POSITION_X as f32, BIRD_START_POSITION_Y as f32),
            velocity: Vector2::new(0.0, 0.0),
        }
    }

    pub fn fly(&mut self) -> () {
        self.velocity = Vector2::up() * BIRD_FLY_SPEED;
    }

    pub fn update(&mut self, dt: f32) -> () {
        self.velocity = self.velocity + Vector2::down() * BIRD_GRAVITY_ACCELERATION_SCALE;
        self.position = self.position + self.velocity * dt;
    }

    pub fn draw(&self, frame: &mut [u8]) -> () {
        let stride = WIDTH as usize * 4;

        let x_position = self.position.x as usize;
        let y_position = self.position.y as usize;
        
        for x in x_position..x_position + BIRD_WIDTH as usize {
            for y in y_position..y_position + BIRD_HEIGHT as usize {
                let idx = y * stride + x as usize * 4;

                if idx > frame.len() { break; }

                frame[idx + 0] = BIRD_R_COLOR;
                frame[idx + 1] = BIRD_G_COLOR;
                frame[idx + 2] = BIRD_B_COLOR;
                frame[idx + 3] = BIRD_A_COLOR;
            }
        }
    }

    pub fn is_passed_pipe(&self, pipe: &Pipe) -> bool {
        self.position.x > pipe.position.x + PIPE_WIDTH as f32
    }
}