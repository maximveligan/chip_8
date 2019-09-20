use std::fmt;
use nybble::Nybble;
use cpu::Ram;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

#[derive(Clone, Copy)]
pub struct Screen {
    pub buffer: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
    pub height: usize,
    pub width: usize,
}

impl fmt::Debug for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut screen_string = "".to_string();
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                if self.buffer[y][x] {
                    screen_string.push_str("*");
                } else {
                    screen_string.push_str(" ");
                }
            }
            screen_string.push_str("\n");
        }
        write!(f, "{}", screen_string)
    }
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            buffer: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
            height: SCREEN_HEIGHT,
            width: SCREEN_WIDTH,
        }
    }

    pub fn draw_nybble(
        &mut self,
        x: u8,
        y: u8,
        ram_index: u16,
        num_bytes: Nybble,
        collision_flag: &mut u8,
        ram: &Ram,
    ) -> Result<(), String> {
        if x > (SCREEN_WIDTH as u8) || y > (SCREEN_HEIGHT as u8) {
            return Err(format!("Out of screen bounds: {0}, {1}", x, y));
        }
        let sprite = ram.retrieve_bytes(ram_index, num_bytes);
        *collision_flag = 0;
        for byte_num in 0..sprite.len() {
            for bit in 0..8 {
                let pixel_val = get_bit(sprite[byte_num], bit)
                    .expect("Iterator went over 8");
                let y_cord = Some((y as usize + byte_num) % (SCREEN_HEIGHT));
                let x_cord = Some((x + bit) as usize % (SCREEN_WIDTH));
                *collision_flag |= (pixel_val
                    & self.buffer[y_cord.expect("Should've gotten an x value")]
                        [x_cord.expect("Should've gotten a y value")])
                    as u8;

                self.buffer[y_cord.expect("Should've gotten an x value")]
                    [x_cord.expect("Should've gotten a y value")] ^= pixel_val
            }
        }

        Ok(())
    }
}

// Note, this will return error if you attempt to pass in a value over 7, as it
// is "out of bounds" for indexing a u8.
fn get_bit(n: u8, b: u8) -> Result<bool, String> {
    if b > 7 {
        return Err(format!("Attempted to pass in a val greater than 7 {}", b));
    }
    Ok((n >> (7 - b)) & 1 == 1)
}
