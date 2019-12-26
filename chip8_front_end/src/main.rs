extern crate chip8;
extern crate sdl2;
use std::env;
use chip8::Chip8;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureAccess;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
// use std::time::{Duration, Instant};

const PIXEL_SIZE: usize = 5;
// const FRAME_RATE: f64 = 60.0; // Measured in frames per second

fn handle_event(event: Event, emu: &mut Chip8) -> bool {
    match event {
        Event::KeyDown {
            keycode: Some(key), ..
        } => {
            if let Some(u_key) = key_to_usize(key) {
                emu.set_ctrl_state(u_key, true).expect("Can't get here");
            }
            false
        }
        Event::KeyUp {
            keycode: Some(key), ..
        } => {
            if let Some(u_key) = key_to_usize(key) {
                emu.set_ctrl_state(u_key, false).expect("Can't get here");
            }
            false
        }
        _ => false,


    }
}

fn key_to_usize(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(1),
        Keycode::Num2 => Some(2),
        Keycode::Num3 => Some(3),
        Keycode::Num4 => Some(0xc),
        Keycode::Q => Some(4),
        Keycode::W => Some(5),
        Keycode::E => Some(6),
        Keycode::R => Some(0xd),
        Keycode::A => Some(7),
        Keycode::S => Some(8),
        Keycode::D => Some(9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = &env::args().nth(1).ok_or("Did not get a rom")?;
    let mut raw_bytes = Vec::new();
    let mut raw_rom = File::open(path)?;
    raw_rom.read_to_end(&mut raw_bytes)?;

    let mut chip8 = Chip8::new(&raw_bytes);

    // let mut clock_speed: f64 = 540.0;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "Res",
            (chip8.cpu.screen.width * PIXEL_SIZE) as u32,
            (chip8.cpu.screen.height * PIXEL_SIZE) as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .accelerated()
        .build()
        .unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture(
            PixelFormatEnum::RGB24,
            TextureAccess::Streaming,
            chip8.cpu.screen.width as u32,
            chip8.cpu.screen.height as u32,
        )
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut framebuffer = Box::new([0; 64 * 32 * 3]);

    loop {
        // let clocks_per_frame = clock_speed / FRAME_RATE;
        // let now = Instant::now();

        for y in 0..chip8.cpu.screen.height {
            for x in 0..chip8.cpu.screen.width {
                let index = ((y * 64) + x) * 3;
                let c = if chip8.cpu.screen.buffer[y][x] { 255 } else { 0 };
                framebuffer[index] = c;
                framebuffer[index + 1] = c;
                framebuffer[index + 2] = c;
            }
        }

        texture.update(None, &(*framebuffer), 64 * 32 * 3).unwrap();
        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
        
        for _ in 0..540 {
            chip8.run_cycle()?
        }

        for event in event_pump.poll_iter() {
            handle_event(event, &mut chip8);
        }
    }
}
