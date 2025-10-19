use crate::config::{BIRD_FLY_SPEED, BIRD_GRAVITY_ACCELERATION_SCALE, BIRD_HEIGHT, BIRD_R_COLOR, BIRD_A_COLOR, BIRD_B_COLOR, BIRD_G_COLOR, BIRD_START_POSITION_X, BIRD_START_POSITION_Y, BIRD_WIDTH, WIDTH, PIPE_WIDTH};
use crate::game::collision_box::CollisionBox;
use crate::game::pipe::Pipe;
use crate::game::vector2::Vector2;

/// Struct for the pipe.
/// A bird's position represents the top left corner of them.
pub struct Bird {
    position: Vector2,
    velocity: Vector2,
    collision_box: CollisionBox
}

impl Bird {
    pub fn new() -> Self {
        let min = Vector2::new(BIRD_START_POSITION_X as f32, BIRD_START_POSITION_Y as f32);
        let max = Vector2::new(
            (BIRD_START_POSITION_X + BIRD_WIDTH) as f32,
            (BIRD_START_POSITION_Y + BIRD_HEIGHT) as f32
        );

        Self {
            position: min,
            velocity: Vector2::new(0.0, 0.0),
            collision_box: CollisionBox::new(min, max),
        }
    }

    pub fn fly(&mut self) -> () {
        self.velocity = Vector2::up() * BIRD_FLY_SPEED;
    }

    pub fn update(&mut self, dt: f32) -> () {
        self.velocity = self.velocity + Vector2::down() * BIRD_GRAVITY_ACCELERATION_SCALE;
        self.position = self.position + self.velocity * dt;

        // Update collision box
        self.collision_box.min = self.collision_box.min + self.velocity * dt;
        self.collision_box.max = self.collision_box.max + self.velocity * dt;
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

    /// Returns whether this bird is past the given pipe.
    pub fn is_passed_pipe(&self, pipe: &Pipe) -> bool {
        self.position.x > pipe.position.x + PIPE_WIDTH as f32
    }

    /// Returns whether this bird collides with the given pipe.
    pub fn collides_with_pipe(&self, pipe: &Pipe) -> bool {
        self.collision_box.collides_with(&pipe.upper_collision_box) ||
        self.collision_box.collides_with(&pipe.lower_collision_box)
    }
}