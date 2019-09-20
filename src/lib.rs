extern crate rand;
extern crate num;
mod cpu;
mod keyboard;
mod nybble;
mod opcode;
mod screen;

use cpu::Cpu;
use opcode::InvalidOpcode;

pub struct Chip8 {
    pub cpu: Cpu,
}

impl Chip8 {
    pub fn new(rom_bytes: &[u8]) -> Chip8 {
        Chip8 {
            cpu: Cpu::new(rom_bytes),
        }
    }

    pub fn emulate_cycles(
        &mut self,
        dt: f64,
        clock_speed: f64,
    ) -> Result<(), InvalidOpcode> {
        if self.cpu.keyboard.wait_press == None {
            if self.cpu.regs.delay != 0 {
                self.cpu.regs.delay -= 1;
            }
            if self.cpu.regs.sound != 0 {
                self.cpu.regs.sound -= 1;
            }
            let mut num_inst = (dt * clock_speed).round() as usize;

            if num_inst <= 1 {
                num_inst = 1;
            }

            for _ in 0..num_inst {
                if self.cpu.keyboard.wait_press == None {
                    self.cpu.step()?;
                }
            }
            Ok(())
        } else {
            Ok(())
        }
    }
}
