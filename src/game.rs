mod bird;
mod collision_box;
mod draw_utils;
mod pipe;
mod score;
mod vector2;

use self::{bird::Bird, pipe::Pipe};
use crate::config::{
    FONT_HEIGHT_PIXEL_MAP_PIXELS, FONT_PIXEL_SIZE_SCREEN_PIXELS, FONT_START_OFFSET, HEIGHT,
    PIPE_SPACING, PIPE_WIDTH, WIDTH,
};
use crate::game::draw_utils::draw_string;
use rand::rngs::ThreadRng;
use score::Score;
use std::process::exit;

#[derive(PartialEq)]
enum GameState {
    MainMenu,
    Playing,
    Dead,
}

pub struct Game {
    game_state: GameState,
    score: Score,
    bird: Bird,
    pipes: Vec<Pipe>,
    rng: ThreadRng,
}

impl Game {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut pipes = Vec::new();
        // Create first pipe to start creation loop.
        pipes.push(Pipe::new(&mut rng));

        Self {
            game_state: GameState::MainMenu,
            score: Score::new(),
            bird: Bird::new(),
            pipes,
            rng,
        }
    }

    fn refresh_state(&mut self) -> () {
        self.rng = rand::rng();
        self.score = Score::new();
        self.bird = Bird::new();
        
        self.pipes = Vec::new();
        // Create first pipe to start creation loop.
        self.pipes.push(Pipe::new(&mut self.rng));
    }

    pub fn update(&mut self, dt: f32) -> () {
        if self.game_state != GameState::Playing {
            return;
        }

        self.bird.update(dt);

        self.check_for_new_pipe();
        for pipe in &mut self.pipes {
            pipe.update(dt);
        }

        self.check_if_bird_passed_pipe();

        self.clean_up_past_pipes();

        if self.check_if_bird_dies() {
            self.game_state = GameState::Dead;
        }
    }

    pub fn draw(&self, frame: &mut [u8]) -> () {
        match self.game_state {
            GameState::MainMenu => self.draw_main_menu(frame),
            GameState::Playing => self.draw_playing(frame),
            GameState::Dead => self.draw_dead(frame),
        }
    }

    pub fn space_bar_hit(&mut self) -> () {
        self.bird.fly();
    }

    pub fn y_key_hit(&mut self) -> () {
        match self.game_state {
            GameState::Playing => return,
            _ => {
                self.refresh_state();
                self.game_state = GameState::Playing;
            }
        }
    }

    pub fn n_key_hit(&mut self) -> () {
        match self.game_state {
            GameState::Playing => return,
            _ => exit(0),
        }
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
    /// or if it is in contact with the ceiling or floor.
    fn check_if_bird_dies(&self) -> bool {
        for pipe in &self.pipes {
            if self.bird.collides_with_pipe(pipe) {
                return true;
            }
        }

        // Purposely not adding BIRD_HEIGHT for some leeway.
        if self.bird.position.y < 0.0 || self.bird.position.y > HEIGHT as f32 {
            return true;
        }

        false
    }

    /// Removes pipes that aren't in the screen anymore.
    fn clean_up_past_pipes(&mut self) -> () {
        let first_pipe = self.pipes.first().unwrap();

        if (first_pipe.position.x + PIPE_WIDTH as f32) < 0.0 {
            self.pipes.remove(0);
        }
    }

    fn draw_playing(&self, frame: &mut [u8]) -> () {
        self.bird.draw(frame);

        for pipe in &self.pipes {
            pipe.draw(frame);
        }

        // Draw score last so that it draws over everything.
        self.score.draw(frame);
    }

    fn draw_main_menu(&self, frame: &mut [u8]) -> () {
        draw_string(
            frame,
            "want to play? (y/n)",
            FONT_START_OFFSET,
            FONT_START_OFFSET,
        );
    }

    fn draw_dead(&self, frame: &mut [u8]) -> () {
        let score_string = format!("you got a score of {}!", self.score.score);
        draw_string(frame, &score_string, FONT_START_OFFSET, FONT_START_OFFSET);

        let play_again_string = format!("play again? (y/n)");
        draw_string(
            frame,
            &play_again_string,
            FONT_START_OFFSET,
            FONT_START_OFFSET + FONT_HEIGHT_PIXEL_MAP_PIXELS * FONT_PIXEL_SIZE_SCREEN_PIXELS,
        );
    }
}
