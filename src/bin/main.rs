extern crate chip_8;
extern crate num;
extern crate piston_window;
use piston_window::*;
use std::env;
use chip_8::Chip8;
use std::error::Error;
use std::fs::File;
use std::io::Read;

const PIXEL_SIZE: f64 = 5.0;

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = &env::args().nth(1).ok_or("Did not get a rom")?;
    let mut raw_bytes = Vec::new();
    let mut raw_rom = File::open(path)?;
    raw_rom.read_to_end(&mut raw_bytes)?;

    let mut chip8 = Chip8::new(&raw_bytes);

    let mut clock_speed: f64 = 540.0;
    let mut pause: bool = false;

    let mut window: PistonWindow = WindowSettings::new(
        "Rust-8 Emulator",
        [
            (chip8.cpu.screen.width * PIXEL_SIZE as usize) as u32,
            (chip8.cpu.screen.height * PIXEL_SIZE as usize) as u32,
        ],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();
    window.set_ups(60);

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                clear([0.5, 0.5, 0.5, 1.0], g);
                for y in 0..chip8.cpu.screen.height {
                    for x in 0..chip8.cpu.screen.width {
                        if chip8.cpu.screen.buffer[y][x] {
                            rectangle(
                                [1.0, 1.0, 1.0, 1.0],
                                [
                                    (x as f64) * PIXEL_SIZE,
                                    (y as f64) * PIXEL_SIZE,
                                    PIXEL_SIZE,
                                    PIXEL_SIZE,
                                ],
                                c.transform,
                                g,
                            );
                        } else {
                            rectangle(
                                [0.0, 0.0, 0.0, 1.0],
                                [
                                    (x as f64) * PIXEL_SIZE,
                                    (y as f64) * PIXEL_SIZE,
                                    PIXEL_SIZE,
                                    PIXEL_SIZE,
                                ],
                                c.transform,
                                g,
                            );
                        }
                    }
                }
            });
        }

        if let Some(up_args) = e.update_args() {
            if !pause {
                chip8.emulate_cycles(up_args.dt, clock_speed)?
            }
        }

        if let Some(k) = e.press_args() {
            match k {
                Button::Keyboard(input) => {
                    if let Some(input) = key_to_usize(input) {
                        chip8
                            .cpu
                            .keyboard
                            .press_key(input, &mut chip8.cpu.regs.v_regs)
                    } else {
                        match input {
                            Key::LeftBracket => {
                                if clock_speed > 0.0 {
                                    clock_speed -= 10.0
                                };
                            }
                            Key::RightBracket => {
                                clock_speed += 10.0;
                            }
                            Key::P => pause = !pause,
                            Key::M => {
                                if pause {
                                    chip8.emulate_cycles(1.0, 1.0)?;
                                };
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }

        if let Some(k) = e.release_args() {
            match k {
                Button::Keyboard(input) => {
                    if let Some(key) = key_to_usize(input) {
                        chip8.cpu.keyboard.release_key(key);
                    }
                }
                _ => (),
            }
        }
    }
    Ok(())
}

fn key_to_usize(key: Key) -> Option<usize> {
    match key {
        Key::D1 => Some(1),
        Key::D2 => Some(2),
        Key::D3 => Some(3),
        Key::D4 => Some(0xc),
        Key::Q => Some(4),
        Key::W => Some(5),
        Key::E => Some(6),
        Key::R => Some(0xd),
        Key::A => Some(7),
        Key::S => Some(8),
        Key::D => Some(9),
        Key::F => Some(0xE),
        Key::Z => Some(0xA),
        Key::X => Some(0),
        Key::C => Some(0xB),
        Key::V => Some(0xF),
        _ => None,
    }
}
