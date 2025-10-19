use crate::config::{DIGITS, FONT_HEIGHT_SCREEN_PIXELS, FONT_PIXEL_SIZE_SCREEN_PIXELS, FONT_WIDTH_SIZE_SCREEN_PIXELS, SCORE_START_OFFSET, UI_A_COLOR, UI_B_COLOR, UI_G_COLOR, UI_R_COLOR, WIDTH};
use std::io::{Error, ErrorKind::InvalidInput};

pub struct Score {
    pub score: u32,
}

impl Score {
    pub fn new() -> Self {
        Self { score: 0 }
    }

    pub fn increase_score(&mut self) -> (){
        self.score += 1;
    }

    pub fn draw(&self, frame: &mut [u8]) -> () {
        let mut score = self.score;
        let offset_scale = FONT_PIXEL_SIZE_SCREEN_PIXELS * FONT_WIDTH_SIZE_SCREEN_PIXELS;

        // Still want to render zero
        if score == 0 {
            draw_digit(frame, 0, SCORE_START_OFFSET, SCORE_START_OFFSET).unwrap();
            return;
        }

        // Compute divisor to get most significant digit first
        let mut div = 1;
        while score / div >= 10 {
            div *= 10;
        }

        let mut x_offset = SCORE_START_OFFSET;

        while div > 0 {
            let digit = score / div;
            draw_digit(frame, digit, x_offset, SCORE_START_OFFSET).unwrap();
            score %= div;
            div /= 10;
            x_offset += offset_scale;
        }
    }
}

/// Draws the given digit at the location (x, y)
/// Where x and y are in pixel coordinates of the given frame.
/// The x and y are the top left corner of the digit.
fn draw_digit(frame: &mut [u8], digit: u32, x: u32, y: u32) -> Result<(), Error> {
    if digit > 9 {
        return Err(Error::new(InvalidInput, "digit must be between 0–9"));
    }

    let pixel_map = &DIGITS[digit as usize];

    for i in 0..FONT_WIDTH_SIZE_SCREEN_PIXELS {
        for j in 0..FONT_HEIGHT_SCREEN_PIXELS {
            let idx = (j * FONT_WIDTH_SIZE_SCREEN_PIXELS + i) as usize;

            if pixel_map[idx] {
                draw_digit_pixel(frame, x, y, i, j);
            }
        }
    }

    Ok(())
}

/// Draws a font pixel at the given location (x, y)
/// Where x and y are in pixel coordinates of the given frame and represent the location of the digit.
/// And where x_pixel and y_pixel are in coordinates of the digit pixel map.
fn draw_digit_pixel(frame: &mut [u8], x: u32, y: u32, x_pixel: u32, y_pixel: u32) -> () {
    // The location of the digit pixel is the top left corner of it.
    // It is in pixel coordinates
    let x_digit_pixel_location = x + x_pixel * FONT_PIXEL_SIZE_SCREEN_PIXELS;
    let y_digit_pixel_location = y + y_pixel * FONT_PIXEL_SIZE_SCREEN_PIXELS;

    let stride = WIDTH * 4;

    for x in x_digit_pixel_location..x_digit_pixel_location + FONT_PIXEL_SIZE_SCREEN_PIXELS {
        for y in y_digit_pixel_location..y_digit_pixel_location + FONT_PIXEL_SIZE_SCREEN_PIXELS {
            let idx = (y * stride + x * 4) as usize;

            if idx > frame.len() { break; }

            frame[idx + 0] = UI_R_COLOR;
            frame[idx + 1] = UI_G_COLOR;
            frame[idx + 2] = UI_B_COLOR;
            frame[idx + 3] = UI_A_COLOR;
        }
    }
}
