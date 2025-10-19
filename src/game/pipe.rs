use crate::config::{WIDTH, HEIGHT, PIPE_WIDTH, PIPE_SPEED, PIPE_A_COLOR, PIPE_B_COLOR, PIPE_G_COLOR, PIPE_R_COLOR, PIPE_GAP_SIZE, PIPE_GAP_BOUND};
use rand::Rng;
use rand_pcg::Pcg32;
use crate::game::collision_box::CollisionBox;
use crate::game::vector2::Vector2;

/// Struct for the pipe.
/// A pipe's position represents the top left corner of them.
pub struct Pipe {
    pub position: Vector2,
    velocity: Vector2,
    y_gap_location: u32,
    pub passed: bool,

    pub upper_collision_box: CollisionBox,
    pub lower_collision_box: CollisionBox,
}

impl Pipe {
    /// On creation, pipes are placed just past the right side of the screen.
    pub fn new(rng: &mut Pcg32) -> Self {
        let y_gap_location = rng.random_range(PIPE_GAP_BOUND..HEIGHT - PIPE_GAP_BOUND - PIPE_GAP_SIZE);

        let min_upper = Vector2::new(WIDTH as f32, 0.0);
        let max_upper = Vector2::new(
            (WIDTH + PIPE_WIDTH) as f32,
            y_gap_location as f32
        );

        let min_lower = Vector2::new(
            WIDTH as f32,
            (y_gap_location + PIPE_GAP_SIZE) as f32
        );
        let max_lower = Vector2::new(
            (WIDTH + PIPE_WIDTH) as f32,
            HEIGHT as f32,
        );

        Self {
            position: min_upper,
            velocity: Vector2::left() * PIPE_SPEED,
            y_gap_location,
            passed: false,
            upper_collision_box: CollisionBox::new(min_upper, max_upper),
            lower_collision_box: CollisionBox::new(min_lower, max_lower),
        }
    }

    pub fn update(&mut self, dt: f32) -> () {
        self.position = self.position + self.velocity * dt;

        // Update collision boxes
        self.upper_collision_box.min = self.upper_collision_box.min + self.velocity * dt;
        self.upper_collision_box.max = self.upper_collision_box.max + self.velocity * dt;

        self.lower_collision_box.min = self.lower_collision_box.min + self.velocity * dt;
        self.lower_collision_box.max = self.lower_collision_box.max + self.velocity * dt;
    }

    pub fn draw(&self, frame: &mut [u8]) -> () {
        let stride = WIDTH as usize * 4;
        let x_position = self.position.x as i32;
        
        for x in x_position..x_position + PIPE_WIDTH as i32 {
            // Don't draw out of bounds
            if x < 0 || x * 4 >= stride as i32 {
                continue;
            }

            for y in 0..HEIGHT as usize {
                if y > self.y_gap_location as usize && y < (self.y_gap_location + PIPE_GAP_SIZE) as usize {
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
