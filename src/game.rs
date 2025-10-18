mod bird;
mod pipe;
mod vector2;
mod ui;

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
}
