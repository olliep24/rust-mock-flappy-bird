pub struct Game {
    score: u32,
}


impl Game {
    pub fn new() -> Self {
        Self { score: 0 }
    }

    pub fn update(&mut self, dt: f32) -> () {
        println!("Update");
    }

    pub fn draw(&self, frame: &mut [u8]) -> () {
        println!("Draw");
    }
}