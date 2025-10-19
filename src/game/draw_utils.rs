use crate::config::{
    DIGITS, FONT_HEIGHT_PIXEL_MAP_PIXELS, FONT_PIXEL_SIZE_SCREEN_PIXELS, FONT_START_OFFSET,
    FONT_WIDTH_SIZE_PIXEL_MAP_PIXELS, LETTERS, SPECIALS, UI_A_COLOR, UI_B_COLOR, UI_G_COLOR,
    UI_R_COLOR, WIDTH,
};
use std::io::Error;
use std::io::ErrorKind::InvalidInput;

/// Draws the given number onto the screen.
pub fn draw_number(frame: &mut [u8], score: u32) -> () {
    let mut score = score;
    let offset = FONT_PIXEL_SIZE_SCREEN_PIXELS * FONT_WIDTH_SIZE_PIXEL_MAP_PIXELS;

    // Still want to render zero
    if score == 0 {
        draw_digit(frame, 0, FONT_START_OFFSET, FONT_START_OFFSET).unwrap();
        return;
    }

    // Compute divisor to get most significant digit first
    let mut div = 1;
    while score / div >= 10 {
        div *= 10;
    }

    let mut x_offset = FONT_START_OFFSET;

    while div > 0 {
        let digit = score / div;
        draw_digit(frame, digit, x_offset, FONT_START_OFFSET).unwrap();
        score %= div;
        div /= 10;
        x_offset += offset;
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

    draw_pixel_map(frame, pixel_map, x, y);

    Ok(())
}

/// Draws the given string at the location (x, y)
/// Where x and y are in pixel coordinates of the given frame.
/// The x and y are the top left corner of the string.
pub fn draw_string(frame: &mut [u8], string: &str, x: u32, y: u32) -> () {
    let offset = FONT_PIXEL_SIZE_SCREEN_PIXELS * FONT_WIDTH_SIZE_PIXEL_MAP_PIXELS;

    let mut x = x;
    for c in string.chars() {
        draw_char(frame, c, x, y);
        x += offset;
    }
}

/// Draws the given character at the location (x, y)
/// Where x and y are in pixel coordinates of the given frame.
/// The x and y are the top left corner of the character.
fn draw_char(frame: &mut [u8], c: char, x: u32, y: u32) -> () {
    if c.is_ascii_digit() {
        draw_digit(frame, c.to_digit(10).unwrap(), x, y).unwrap();
        return;
    }

    let pixel_map;

    // Handle special character edge cases first
    match c {
        '(' => pixel_map = &SPECIALS[0],
        ')' => pixel_map = &SPECIALS[1],
        '!' => pixel_map = &SPECIALS[2],
        '?' => pixel_map = &SPECIALS[3],
        '/' => pixel_map = &SPECIALS[4],
        ' ' => return,
        _ => {
            let idx = (c as u8 - b'a') as usize; // b'a' == 97u8
            pixel_map = &LETTERS[idx];
        }
    }

    draw_pixel_map(frame, pixel_map, x, y);
}

/// Draws the given pixel map at the location (x, y)
/// Where x and y are in pixel coordinates of the given frame.
/// The x and y are the top left corner of the pixel map.
fn draw_pixel_map(frame: &mut [u8], pixel_map: &[bool; 63], x: u32, y: u32) -> () {
    for i in 0..FONT_WIDTH_SIZE_PIXEL_MAP_PIXELS {
        for j in 0..FONT_HEIGHT_PIXEL_MAP_PIXELS {
            let idx = (j * FONT_WIDTH_SIZE_PIXEL_MAP_PIXELS + i) as usize;

            if pixel_map[idx] {
                draw_font_pixel(frame, x, y, i, j);
            }
        }
    }
}

/// Draws a font pixel at the given location (x, y)
/// Where x and y are in pixel coordinates of the given frame and represent the location of the digit.
/// And where x_pixel and y_pixel are in coordinates of the digit pixel map.
fn draw_font_pixel(frame: &mut [u8], x: u32, y: u32, x_pixel: u32, y_pixel: u32) -> () {
    // The location of the digit pixel is the top left corner of it.
    // It is in pixel coordinates
    let x_digit_pixel_location = x + x_pixel * FONT_PIXEL_SIZE_SCREEN_PIXELS;
    let y_digit_pixel_location = y + y_pixel * FONT_PIXEL_SIZE_SCREEN_PIXELS;

    let stride = WIDTH * 4;

    for x in x_digit_pixel_location..x_digit_pixel_location + FONT_PIXEL_SIZE_SCREEN_PIXELS {
        for y in y_digit_pixel_location..y_digit_pixel_location + FONT_PIXEL_SIZE_SCREEN_PIXELS {
            let idx = (y * stride + x * 4) as usize;

            if idx > frame.len() {
                break;
            }

            frame[idx + 0] = UI_R_COLOR;
            frame[idx + 1] = UI_G_COLOR;
            frame[idx + 2] = UI_B_COLOR;
            frame[idx + 3] = UI_A_COLOR;
        }
    }
}
