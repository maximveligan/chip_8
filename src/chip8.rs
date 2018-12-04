extern crate rand;
use nybble::Nybble;
use opcode::Opcode;
use opcode::InvalidOpcode;
use opcode::NoArg;
use opcode::OneArg;
use opcode::TwoArg;
use opcode::ThreeArg;
use std::fmt;
use num::ToPrimitive;
use std::fs::File;
use std::io::prelude::*;

const SPR_ZERO_START: u16 = 0000;
const SPR_ONE_START: u16 = 0005;
const SPR_TWO_START: u16 = 0010;
const SPR_THREE_START: u16 = 0015;
const SPR_FOUR_START: u16 = 0020;
const SPR_FIVE_START: u16 = 0025;
const SPR_SIX_START: u16 = 0030;
const SPR_SEVEN_START: u16 = 0035;
const SPR_EIGHT_START: u16 = 0040;
const SPR_NINE_START: u16 = 0045;
const SPR_A_START: u16 = 0050;
const SPR_B_START: u16 = 0055;
const SPR_C_START: u16 = 0060;
const SPR_D_START: u16 = 0065;
const SPR_E_START: u16 = 0070;
const SPR_F_START: u16 = 1075;

const SPR_ZERO: [u8; 5] = [0xF0, 0x90, 0x90, 0x90, 0xF0];
const SPR_ONE: [u8; 5] = [0x20, 0x60, 0x20, 0x20, 0x70];
const SPR_TWO: [u8; 5] = [0xF0, 0x10, 0xF0, 0x80, 0xF0];
const SPR_THREE: [u8; 5] = [0xF0, 0x10, 0xF0, 0x10, 0xF0];
const SPR_FOUR: [u8; 5] = [0x90, 0x90, 0xF0, 0x10, 0x10];
const SPR_FIVE: [u8; 5] = [0xF0, 0x80, 0xF0, 0x10, 0xF0];
const SPR_SIX: [u8; 5] = [0xF0, 0x80, 0xF0, 0x90, 0xF0];
const SPR_SEVEN: [u8; 5] = [0xF0, 0x10, 0x20, 0x40, 0x40];
const SPR_EIGHT: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0xF0];
const SPR_NINE: [u8; 5] = [0xF0, 0x90, 0xF0, 0x10, 0xF0];
const SPR_A: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0x90];
const SPR_B: [u8; 5] = [0xE0, 0x90, 0xE0, 0x90, 0xE0];
const SPR_C: [u8; 5] = [0xF0, 0x80, 0x80, 0x80, 0xF0];
const SPR_D: [u8; 5] = [0xE0, 0x90, 0x90, 0x90, 0xE0];
const SPR_E: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0xF0];
const SPR_F: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0x80];

const FLAG_REG: usize = 0xF;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

#[derive(Debug, Clone, Copy)]
pub struct Registers {
    pc: ProgramCounter,
    delay: u8,
    sound: u8,
    sp: u8,
    i_reg: u16,
    pub v_regs: [u8; 16],
}

impl Registers {
    fn new() -> Registers {
        let chip_8_adrr = 0x200;
        Registers {
            pc: ProgramCounter(chip_8_adrr),
            delay: 0,
            sound: 0,
            i_reg: 0,
            sp: 0,
            v_regs: [0; 16],
        }
    }
}

#[derive(Clone, Copy)]
struct Ram([u8; 0xFFF]);

impl fmt::Debug for Ram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ram_string = "".to_string();
        for index in 0..0xFFF {
            if index % 64 == 0 {
                ram_string.push_str("\n");
            }
            ram_string.push_str(&self.0[index].to_string());
        }
        write!(f, "Ram Dump\n{}", ram_string)
    }
}

impl Ram {
    fn initialize_ram(path: &str) -> Result<Ram, String> {
        let mut ram = Ram { 0: [0; 0xFFF] };
        ram.load_digit_data();
        match ram.load_rom(path) {
            Ok(_) => Ok(ram),
            Err(err) => Err(err),
        }
    }

    fn load_digit_data(&mut self) {
        self.0[0000..0005].copy_from_slice(&SPR_ZERO);
        self.0[0005..0010].copy_from_slice(&SPR_ONE);
        self.0[0010..0015].copy_from_slice(&SPR_TWO);
        self.0[0015..0020].copy_from_slice(&SPR_THREE);
        self.0[0020..0025].copy_from_slice(&SPR_FOUR);
        self.0[0025..0030].copy_from_slice(&SPR_FIVE);
        self.0[0030..0035].copy_from_slice(&SPR_SIX);
        self.0[0035..0040].copy_from_slice(&SPR_SEVEN);
        self.0[0040..0045].copy_from_slice(&SPR_EIGHT);
        self.0[0045..0050].copy_from_slice(&SPR_NINE);
        self.0[0050..0055].copy_from_slice(&SPR_A);
        self.0[0055..0060].copy_from_slice(&SPR_B);
        self.0[0060..0065].copy_from_slice(&SPR_C);
        self.0[0065..0070].copy_from_slice(&SPR_D);
        self.0[0070..0075].copy_from_slice(&SPR_E);
        self.0[0075..0080].copy_from_slice(&SPR_F);
    }

    fn retrieve_bytes(self, index: u16, amount: Nybble) -> Vec<u8> {
        let mut byte_vec = Vec::new();
        for i in index as usize
            ..(index as usize + amount.to_usize().expect("Check usize"))
        {
            byte_vec.push(self.0[i]);
        }
        byte_vec
    }

    fn load_rom(&mut self, file: &str) -> Result<(), String> {
        match File::open(file) {
            Ok(mut rom) => {
                let mut raw_bytes = Vec::new();
                rom.read_to_end(&mut raw_bytes)
                    .expect("Something went wrong while reading the rom");
                self.0[0x200..0x200 + raw_bytes.len()]
                    .copy_from_slice(&raw_bytes);
                Ok(())
            }
            Err(e) => Err(format!("No such file: {0}. {1}", file, e)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ProgramCounter(u16);

impl ProgramCounter {
    fn update(&mut self) {
        self.0 += 2;
    }
    fn set_addr(&mut self, addr: u16) {
        self.0 = addr;
    }
    fn get_addr(&self) -> u16 {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
struct Stack([u16; 16]);

impl Stack {
    fn new() -> Stack {
        Stack { 0: [0; 16] }
    }
    fn push(&mut self, sp: &mut u8, pc: &ProgramCounter) -> Result<(), String> {
        *sp = *sp + 1;
        if *sp > 15 {
            return Err("Stack overflow".to_string());
        }
        self.0[*sp as usize] = pc.get_addr();
        Ok(())
    }

    fn pop(&self, sp: &mut u8) -> Result<ProgramCounter, String> {
        let temp = ProgramCounter(self.0[*sp as usize]);
        if *sp <= 0 {
            return Err("Stack pointer cannot go below 0".to_string());
        }
        *sp = *sp - 1;
        Ok(temp)
    }
}

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
    fn new() -> Screen {
        Screen {
            buffer: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
            height: SCREEN_HEIGHT,
            width: SCREEN_WIDTH,
        }
    }

    fn draw_nybble(
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

#[derive(Debug, Clone, Copy)]
pub struct Keyboard {
    key_buffer: [bool; 0xF + 1],
    wait_press: Option<u8>,
}

impl Keyboard {
    fn new() -> Keyboard {
        Keyboard {
            key_buffer: [false; 0xF + 1],
            wait_press: None,
        }
    }

    pub fn press_key(&mut self, key: usize, vreg: &mut [u8; 16]) {
        self.key_buffer[key] = true;
        if self.wait_press != None {
            vreg[self.wait_press.unwrap() as usize] = key as u8;
            self.wait_press = None;
        }
    }

    pub fn release_key(&mut self, key: usize) {
        self.key_buffer[key] = false;
    }
}

pub struct Chip8 {
    pub regs: Registers,
    ram: Ram,
    stack: Stack,
    pub screen: Screen,
    pub keyboard: Keyboard,
}

impl Chip8 {
    pub fn new(rom_path: &str) -> Result<Chip8, String> {
        Ok(Chip8 {
        ram: match Ram::initialize_ram(&rom_path) {
            Ok(r) => r,
            Err(e) => return Err(e),
        },
        regs: Registers::new(),
        stack: Stack::new(),
        screen: Screen::new(),
        keyboard: Keyboard::new(),
        })
    }

    fn execute(
        &mut self,
        opcode: Opcode,
        debug: bool,
    ) -> Result<(), InvalidOpcode> {
        if debug {
            println!("{:?}", opcode);
        }
        match opcode {
            Opcode::NoArg(NoArg::ClearScreen) => {
                self.screen.buffer.iter_mut().for_each(|inner_array| {
                    inner_array.iter_mut().for_each(|pixel| *pixel = false)
                });
                self.regs.pc.update();
                Ok(())
            }
            Opcode::NoArg(NoArg::ReturnSubrt) => match self.stack.pop(&mut self.regs.sp) {
                Ok(pc) => {
                    self.regs.pc = pc;
                    self.regs.pc.update();
                    Ok(())
                }
                Err(err) => Err(InvalidOpcode::StackUnderflow(err, opcode)),
            },

            Opcode::OneArg(OneArg::SkipIfVx(arg)) => {
                if self.keyboard.key_buffer
                    [self.regs.v_regs[arg.to_usize().expect("Check usize")] as usize]
                {
                    self.regs.pc.update();
                }
                self.regs.pc.update();
                Ok(())
            }
            Opcode::OneArg(OneArg::SkipIfNVx(arg)) => {
                if !self.keyboard.key_buffer
                    [self.regs.v_regs[arg.to_usize().expect("Check usize")] as usize]
                {
                    self.regs.pc.update();
                }
                self.regs.pc.update();
                Ok(())
            }
            Opcode::OneArg(OneArg::SetVxDT(arg)) => {
                self.regs.v_regs[arg.to_usize().expect("Check usize")] = self.regs.delay;
                self.regs.pc.update();
                Ok(())
            }
            Opcode::OneArg(OneArg::WaitForKey(arg)) => {
                self.keyboard.wait_press = Some(arg.to_u8().expect("Check u8"));
                self.regs.pc.update();
                Ok(())
            }
            Opcode::OneArg(OneArg::SetDT(arg)) => {
                self.regs.delay = self.regs.v_regs[arg.to_usize().expect("Check usize")];
                self.regs.pc.update();
                Ok(())
            }
            Opcode::OneArg(OneArg::SetST(arg)) => {
                self.regs.sound = self.regs.v_regs[arg.to_usize().expect("Check usize")];
                self.regs.pc.update();
                Ok(())
            }
            Opcode::OneArg(OneArg::SetI(arg)) => {
                self.regs.i_reg +=
                    (self.regs.v_regs[arg.to_usize().expect("Check usize")]) as u16;
                self.regs.pc.update();

                Ok(())
            }
            Opcode::OneArg(OneArg::SetSpriteI(arg)) => match i_eq_spr_digit_vx(
                self.regs.v_regs[arg.to_usize().expect("Check usize")],
                &mut self.regs.i_reg,
            ) {
                Ok(_) => {
                    self.regs.pc.update();
                    Ok(())
                }
                Err(err) => Err(InvalidOpcode::NoSuchDigitSprite(err, opcode)),
            },
            Opcode::OneArg(OneArg::StoreDecVx(arg)) => {
                let tmp = self.regs.v_regs[arg.to_usize().expect("Check usize")];
                self.ram.0[self.regs.i_reg as usize] = tmp / 100;
                self.ram.0[(self.regs.i_reg + 1) as usize] = (tmp % 100) / 10;
                self.ram.0[(self.regs.i_reg + 2) as usize] = tmp % 10;
                self.regs.pc.update();
                Ok(())
            }
            Opcode::OneArg(OneArg::StoreV0Vx(arg)) => {
                for index in 0..=arg.to_u64().expect("Check to_u64") {
                    self.ram.0[(self.regs.i_reg + index as u16) as usize] = self.regs.v_regs[index as usize]
                }
                self.regs.pc.update();
                Ok(())
            }
            Opcode::OneArg(OneArg::ReadV0Vx(arg)) => {
                for index in 0..=arg.to_usize().expect("Check usize") {
                    self.regs.v_regs[index as usize] = self.ram.0[(self.regs.i_reg + index as u16) as usize];
                }
                self.regs.pc.update();
                Ok(())
            }
            Opcode::TwoArg(TwoArg::SkipEqVxVy(arg)) => {
                if self.regs.v_regs[arg.x().to_usize().expect("Check usize")]
                    == self.regs.v_regs[arg.y().to_usize().expect("Check usize")]
                {
                    self.regs.pc.update();
                }

                self.regs.pc.update();
                Ok(())
            }
            Opcode::TwoArg(TwoArg::VxEqVy(arg)) => {
                self.regs.v_regs[arg.x().to_usize().expect("Check usize")] =
                    self.regs.v_regs[arg.y().to_usize().expect("Check usize")];
                self.regs.pc.update();
                Ok(())
            }
            Opcode::TwoArg(TwoArg::VxOREqVy(arg)) => {
                self.regs.v_regs[arg.x().to_usize().expect("Check usize")] |=
                    self.regs.v_regs[arg.y().to_usize().expect("Check usize")];
                self.regs.pc.update();
                Ok(())
            }
            Opcode::TwoArg(TwoArg::VxANDEqVy(arg)) => {
                self.regs.v_regs[arg.x().to_usize().expect("Check usize")] &=
                    self.regs.v_regs[arg.y().to_usize().expect("Check usize")];
                self.regs.pc.update();
                Ok(())
            }
            Opcode::TwoArg(TwoArg::VxXOREqVy(arg)) => {
                self.regs.v_regs[arg.x().to_usize().expect("Check usize")] ^=
                    self.regs.v_regs[arg.y().to_usize().expect("Check usize")];
                self.regs.pc.update();
                Ok(())
            }
            Opcode::TwoArg(TwoArg::VxPlusEqVySetF(arg)) => {
                let (x, flag) = self.regs.v_regs
                    [arg.x().to_usize().expect("Check usize")]
                    .overflowing_add(
                        self.regs.v_regs[arg.y().to_usize().expect("Check usize")],
                    );
                self.regs.v_regs[FLAG_REG] = flag as u8;
                self.regs.v_regs[arg.x().to_usize().expect("Check usize")] = x;
                self.regs.pc.update();
                Ok(())
            }
            Opcode::TwoArg(TwoArg::VxSubEqVySetF(arg)) => {
                let (x, flag) = self.regs.v_regs
                    [arg.x().to_usize().expect("Check usize")]
                    .overflowing_sub(
                        self.regs.v_regs[arg.y().to_usize().expect("Check usize")],
                    );
                self.regs.v_regs[FLAG_REG] = (!flag) as u8;
                self.regs.v_regs[arg.x().to_usize().expect("Check usize")] = x;
                self.regs.pc.update();
                Ok(())
            }
            Opcode::TwoArg(TwoArg::ShiftVxR(arg)) => {
                self.regs.v_regs[FLAG_REG] = (0b00000001
                    & self.regs.v_regs[arg.x().to_usize().expect("Check usize")]
                    == 0b00000001) as u8;
                self.regs.v_regs[arg.x().to_usize().expect("Check usize")] =
                    self.regs.v_regs[arg.x().to_usize().expect("Check usize")] >> 1;
                self.regs.pc.update();
                Ok(())
            }
            Opcode::TwoArg(TwoArg::VxEqVySubVxSetF(arg)) => {
                let (y, flag) = self.regs.v_regs
                    [arg.y().to_usize().expect("Check usize")]
                    .overflowing_sub(
                        self.regs.v_regs[arg.x().to_usize().expect("Check usize")],
                    );
                self.regs.v_regs[FLAG_REG] = (!flag) as u8;
                self.regs.v_regs[arg.x().to_usize().expect("Check usize")] = y;
                self.regs.pc.update();
                Ok(())
            }
            Opcode::TwoArg(TwoArg::ShiftVxL(arg)) => {
                self.regs.v_regs[FLAG_REG] = (0b10000000
                    & self.regs.v_regs[arg.x().to_usize().expect("Check usize")])
                    >> 7 as u8;
                self.regs.v_regs[arg.x().to_usize().expect("Check usize")] =
                    self.regs.v_regs[arg.x().to_usize().expect("Check usize")] << 1;
                self.regs.pc.update();
                Ok(())
            }
            Opcode::TwoArg(TwoArg::SkipVxNEqVy(arg)) => {
                if self.regs.v_regs[arg.x().to_usize().expect("Check usize")]
                    != self.regs.v_regs[arg.y().to_usize().expect("Check usize")]
                {
                    self.regs.pc.update();
                }

                self.regs.pc.update();
                Ok(())
            }
            Opcode::ThreeArg(ThreeArg::JumpToCodeRout(_)) => {
                self.regs.pc.update();
                Ok(())
            }
            Opcode::ThreeArg(ThreeArg::JumpToAddr(arg)) => {
                self.regs.pc.set_addr(arg.to_addr());
                Ok(())
            }
            Opcode::ThreeArg(ThreeArg::CallSubAt(arg)) => {
                match self.stack.push(&mut self.regs.sp, &self.regs.pc) {
                    Ok(_) => {
                        self.regs.pc.set_addr(arg.to_addr());
                        Ok(())
                    }
                    Err(err) => Err(InvalidOpcode::StackOverflow(err, opcode)),
                }
            }
            Opcode::ThreeArg(ThreeArg::SkipVxEqKK(arg)) => {
                if self.regs.v_regs[arg.x().to_usize().expect("Check usize")]
                    == arg.get_byte()
                {
                    self.regs.pc.update();
                }

                self.regs.pc.update();
                Ok(())
            }
            Opcode::ThreeArg(ThreeArg::SkipVxNEqKK(arg)) => {
                if self.regs.v_regs[arg.x().to_usize().expect("Check usize")]
                    != arg.get_byte()
                {
                    self.regs.pc.update();
                }

                self.regs.pc.update();
                Ok(())
            }
            Opcode::ThreeArg(ThreeArg::SetVxKK(arg)) => {
                self.regs.v_regs[arg.x().to_usize().expect("Check usize") as usize] =
                    arg.get_byte();
                self.regs.pc.update();
                Ok(())
            }
            Opcode::ThreeArg(ThreeArg::VxEqVxPlusKK(arg)) => {
                self.regs.v_regs[arg.x().to_usize().expect("Check usize")] = self.regs.v_regs
                    [arg.x().to_usize().expect("Check usize")]
                    .overflowing_add(arg.get_byte())
                    .0;
                self.regs.pc.update();
                Ok(())
            }
            Opcode::ThreeArg(ThreeArg::SetIToNNN(arg)) => {
                self.regs.i_reg = arg.to_addr();
                self.regs.pc.update();
                Ok(())
            }
            Opcode::ThreeArg(ThreeArg::PCEqNNNPlusV0(arg)) => {
                let sum = (self.regs.v_regs[0] as usize) + arg.to_addr() as usize;
                if sum > 0xffe || sum < 0x200 {
                    return Err(InvalidOpcode::OutOfBoundsAddress(
                        "Out of bounds program counter".to_string(),
                        opcode,
                    ));
                }
                self.regs.pc.set_addr(sum as u16);
                Ok(())
            }
            Opcode::ThreeArg(ThreeArg::VxEqRandANDKK(arg)) => {
                self.regs.v_regs[arg.x().to_usize().expect("Check usize")] =
                    arg.get_byte() & rand::random::<u8>();
                self.regs.pc.update();
                Ok(())
            }
            Opcode::ThreeArg(ThreeArg::DrawVxVyNib(arg)) => match self.screen
                .draw_nybble(
                    self.regs.v_regs[arg.x().to_usize().expect("Check usize")],
                    self.regs.v_regs[arg.y().to_usize().expect("Check usize")],
                    self.regs.i_reg,
                    Nybble::new([arg.last_nybble()]),
                    &mut self.regs.v_regs[FLAG_REG],
                    &mut self.ram,
                ) {
                Ok(_) => {
                    self.regs.pc.update();
                    Ok(())
                }
                Err(err) => Err(InvalidOpcode::OutOfScreenBounds(err, opcode)),
            },
        }
    }

    pub fn emulate_cycles(
        &mut self,
        dt: f64,
        clock_speed: f64,
    ) -> Result<(), InvalidOpcode> {
        if self.keyboard.wait_press == None {
            if self.regs.delay != 0 {
                self.regs.delay -= 1;
            }
            if self.regs.sound != 0 {
                self.regs.sound -= 1;
            }
            let mut num_inst = (dt * clock_speed).round() as usize;

            if num_inst <= 1 {
                num_inst = 1;
            }

            for _ in 0..num_inst {
                if self.keyboard.wait_press == None {
                    let op = Opcode::decode_op(fetch_opcode(&self.regs.pc, &self.ram))?;
                    match self.execute(op, true) {
                        Ok(_) => (),
                        Err(err) => return Err(err),
                    };
                }
            }
            Ok(())
        } else {
            Ok(())
        }
    }
}

fn fetch_opcode(pc: &ProgramCounter, ram: &Ram) -> u16 {
    let l_byte: u8 = ram.0[pc.get_addr() as usize];
    let r_byte: u8 = ram.0[(pc.get_addr() + 1) as usize];
    ((l_byte as u16) << 8) | (r_byte as u16)
}


fn i_eq_spr_digit_vx(v_reg: u8, i_reg: &mut u16) -> Result<(), String> {
    match v_reg {
        0x0 => Ok(*i_reg = SPR_ZERO_START),
        0x1 => Ok(*i_reg = SPR_ONE_START),
        0x2 => Ok(*i_reg = SPR_TWO_START),
        0x3 => Ok(*i_reg = SPR_THREE_START),
        0x4 => Ok(*i_reg = SPR_FOUR_START),
        0x5 => Ok(*i_reg = SPR_FIVE_START),
        0x6 => Ok(*i_reg = SPR_SIX_START),
        0x7 => Ok(*i_reg = SPR_SEVEN_START),
        0x8 => Ok(*i_reg = SPR_EIGHT_START),
        0x9 => Ok(*i_reg = SPR_NINE_START),
        0xa => Ok(*i_reg = SPR_A_START),
        0xb => Ok(*i_reg = SPR_B_START),
        0xc => Ok(*i_reg = SPR_C_START),
        0xd => Ok(*i_reg = SPR_D_START),
        0xe => Ok(*i_reg = SPR_E_START),
        0xf => Ok(*i_reg = SPR_F_START),
        _ => Err(format!("Value {} is not a valid digit sprite!", v_reg)),
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
