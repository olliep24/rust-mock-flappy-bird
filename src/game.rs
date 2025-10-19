mod bird;
mod pipe;
mod vector2;
mod ui;
mod collision_box;

use rand::{SeedableRng};
use rand_pcg::Pcg32;

use crate::config::{PIPE_SPACING, PIPE_WIDTH, SEED, WIDTH};
use crate::game::ui::score::Score;
use self::{bird::Bird, pipe::Pipe};

pub struct Game {
    score: Score,
    bird: Bird,
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
            score: Score::new(),
            bird: Bird::new(),
            pipes,
            rng,
        }
    }

    pub fn update(&mut self, dt: f32) -> () {
        self.bird.update(dt);
        
        self.check_for_new_pipe();
        for pipe in &mut self.pipes {
            pipe.update(dt);
        }

        self.check_if_bird_passed_pipe();

        if self.check_if_bird_dies() {
            println!("You finished with score: {}", self.score.score);
            std::process::exit(0);
        }
    }

    pub fn draw(&self, frame: &mut [u8]) -> () {
        self.bird.draw(frame);

        for pipe in &self.pipes {
            pipe.draw(frame);
        }

        // Draw score last so that it draws over everything.
        self.score.draw(frame);
    }

    pub fn space_bar_hit(&mut self) -> () {
        self.bird.fly();
    }

    fn check_for_new_pipe(&mut self) -> () {
        let last_pipe = self.pipes.last().unwrap();

        if last_pipe.position.x as u32 + PIPE_WIDTH + PIPE_SPACING < WIDTH {
            self.pipes.push(Pipe::new(&mut self.rng));
        }
    }

    fn check_if_bird_passed_pipe(&mut self) -> () {
        for pipe in &mut self.pipes {
            if pipe.passed {
                continue;
            }

            if self.bird.is_passed_pipe(pipe) {
                pipe.passed = true;
                self.score.increase_score();

                // Ok to break because bird should only pass one at a time.
                break;
            }
        }
    }

    /// Returns whether the bird should die.
    /// The bird dies if it is in contact with any of the pipes
    fn check_if_bird_dies(&self) -> bool {
        for pipe in &self.pipes {
            if self.bird.collides_with_pipe(pipe) {
                return true;
            }
        }
        false
    }
}
