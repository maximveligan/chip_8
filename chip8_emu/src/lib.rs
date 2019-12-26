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

    pub fn run_cycle(
        &mut self,
    ) -> Result<(), InvalidOpcode> {
        if self.cpu.keyboard.wait_press == None {
            self.cpu.step()
        } else {
            Ok(())
        }
    }

    pub fn decrement_delay(&mut self) {
        if self.cpu.regs.delay != 0 {
            self.cpu.regs.delay -= 1;
        }
    }
    
    pub fn decrement_sound(&mut self) {
        if self.cpu.regs.sound != 0 {
            self.cpu.regs.sound -= 1;
        }
    }

    pub fn set_ctrl_state(&mut self, button: usize, pressed: bool) -> Result<(), String> {
        if pressed {
            self.cpu.keyboard.press_key(button, &mut self.cpu.regs.v_regs)?
        } else {
            self.cpu.keyboard.release_key(button)?
        }
        Ok(())
    }
}
