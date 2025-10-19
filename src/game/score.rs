use crate::game::draw_utils::draw_number;

pub struct Score {
    pub score: u32,
}

impl Score {
    pub fn new() -> Self {
        Self { score: 0 }
    }

    pub fn increase_score(&mut self) -> () {
        self.score += 1;
    }

    pub fn draw(&self, frame: &mut [u8]) -> () {
        draw_number(frame, self.score);
    }
}
