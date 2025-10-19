// Clock
pub const FIXED_DT: f32 = 1.0 / 120.0;

// Window size
pub const WIDTH: u32 = 900;
pub const HEIGHT: u32 = 600;

// Game constants
pub const SEED: u64 = 2;

// Pipe constants
pub const PIPE_SPEED: f32 = 80.0;
pub const PIPE_WIDTH: u32 = 30;
pub const PIPE_SPACING: u32 = 200;
pub const PIPE_GAP_BOUND: u32 = 30;
pub const PIPE_GAP_SIZE: u32 = 125;

pub const PIPE_R_COLOR: u8 = 0xFF;
pub const PIPE_G_COLOR: u8 = 0xFF;
pub const PIPE_B_COLOR: u8 = 0xFF;
pub const PIPE_A_COLOR: u8 = 0xFF;

// Bird constants
pub const BIRD_START_POSITION_X: u32 = 400;
pub const BIRD_START_POSITION_Y: u32 = 200;
pub const BIRD_FLY_SPEED: f32 = 200.0;
pub const BIRD_GRAVITY_ACCELERATION_SCALE: f32 = 2.0;

pub const BIRD_HEIGHT: u32 = 30;
pub const BIRD_WIDTH: u32 = 30;

pub const BIRD_R_COLOR: u8 = 0xF0;
pub const BIRD_G_COLOR: u8 = 0xDC;
pub const BIRD_B_COLOR: u8 = 0x78;
pub const BIRD_A_COLOR: u8 = 0xFF;

// UI constants
pub const FONT_PIXEL_SIZE_SCREEN_PIXELS: u32 = 6;
pub const FONT_WIDTH_SIZE_SCREEN_PIXELS: u32 = 7;
pub const FONT_HEIGHT_SCREEN_PIXELS: u32 = 9;
pub const SCORE_START_OFFSET: u32 = 50;

pub const UI_R_COLOR: u8 = 0x78;
pub const UI_G_COLOR: u8 = 0xB4;
pub const UI_B_COLOR: u8 = 0xFF;
pub const UI_A_COLOR: u8 = 0xFF;

// Pixel maps for digits
// Helper macros
macro_rules! b { (0) => { false }; (1) => { true }; }
macro_rules! digit {
    ($($n:tt),+ $(,)?) => { [$(b!($n)),+] };
}

// Pixel map stored in row-major form.
pub const DIGITS: [[bool; 63]; 10] = [
    // 0
    digit!(
        0,0,0,0,0,0,0,
        0,0,1,1,1,0,0,
        0,1,0,0,0,1,0,
        0,1,0,0,0,1,0,
        0,1,0,0,0,1,0,
        0,1,0,0,0,1,0,
        0,1,0,0,0,1,0,
        0,0,1,1,1,0,0,
        0,0,0,0,0,0,0,
    ),

    // 1
    digit!(
        0,0,0,0,0,0,0,
        0,0,0,1,0,0,0,
        0,0,1,1,0,0,0,
        0,0,0,1,0,0,0,
        0,0,0,1,0,0,0,
        0,0,0,1,0,0,0,
        0,0,0,1,0,0,0,
        0,0,1,1,1,0,0,
        0,0,0,0,0,0,0,
    ),

    // 2
    digit!(
        0,0,0,0,0,0,0,
        0,0,1,1,1,0,0,
        0,1,0,0,0,1,0,
        0,0,0,0,0,1,0,
        0,0,0,0,1,0,0,
        0,0,0,1,0,0,0,
        0,0,1,0,0,0,0,
        0,1,1,1,1,1,0,
        0,0,0,0,0,0,0,
    ),

    // 3
    digit!(
        0,0,0,0,0,0,0,
        0,0,1,1,1,0,0,
        0,1,0,0,0,1,0,
        0,0,0,0,0,1,0,
        0,0,1,1,1,0,0,
        0,0,0,0,0,1,0,
        0,1,0,0,0,1,0,
        0,0,1,1,1,0,0,
        0,0,0,0,0,0,0,
    ),

    // 4
    digit!(
        0,0,0,0,0,0,0,
        0,0,0,1,1,0,0,
        0,0,1,0,1,0,0,
        0,1,0,0,1,0,0,
        0,1,1,1,1,1,0,
        0,0,0,0,1,0,0,
        0,0,0,0,1,0,0,
        0,0,0,0,1,0,0,
        0,0,0,0,0,0,0,
    ),

    // 5
    digit!(
        0,0,0,0,0,0,0,
        0,1,1,1,1,1,0,
        0,1,0,0,0,0,0,
        0,1,1,1,1,0,0,
        0,0,0,0,0,1,0,
        0,0,0,0,0,1,0,
        0,1,0,0,0,1,0,
        0,0,1,1,1,0,0,
        0,0,0,0,0,0,0,
    ),

    // 6
    digit!(
        0,0,0,0,0,0,0,
        0,0,1,1,1,0,0,
        0,1,0,0,0,1,0,
        0,1,0,0,0,0,0,
        0,1,1,1,1,0,0,
        0,1,0,0,0,1,0,
        0,1,0,0,0,1,0,
        0,0,1,1,1,0,0,
        0,0,0,0,0,0,0,
    ),

    // 7
    digit!(
        0,0,0,0,0,0,0,
        0,1,1,1,1,1,0,
        0,0,0,0,0,1,0,
        0,0,0,0,1,0,0,
        0,0,0,1,0,0,0,
        0,0,0,1,0,0,0,
        0,0,0,1,0,0,0,
        0,0,0,1,0,0,0,
        0,0,0,0,0,0,0,
    ),

    // 8
    digit!(
        0,0,0,0,0,0,0,
        0,0,1,1,1,0,0,
        0,1,0,0,0,1,0,
        0,1,0,0,0,1,0,
        0,0,1,1,1,0,0,
        0,1,0,0,0,1,0,
        0,1,0,0,0,1,0,
        0,0,1,1,1,0,0,
        0,0,0,0,0,0,0,
    ),

    // 9
    digit!(
        0,0,0,0,0,0,0,
        0,0,1,1,1,0,0,
        0,1,0,0,0,1,0,
        0,1,0,0,0,1,0,
        0,0,1,1,1,1,0,
        0,0,0,0,0,1,0,
        0,1,0,0,0,1,0,
        0,0,1,1,1,0,0,
        0,0,0,0,0,0,0,
    ),
];

