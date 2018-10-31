extern crate num;
extern crate rand;
extern crate piston_window;
use num::ToPrimitive;
use piston_window::WindowSettings;
use piston_window::PistonWindow;
use piston_window::RenderEvent;
use piston_window::PressEvent;
use piston_window::*;
use std::fmt;
use nybble::Nybble;
use opcode::Opcode;
use opcode::InvalidOpcode;
use opcode::NoArg;
use opcode::OneArg;
use opcode::TwoArg;
use opcode::ThreeArg;
use std::fs::File;
use std::io::prelude::*;
use std::env;

mod nybble;
mod opcode;

const CLOCK_SPEED: f64 = 540.0;
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
struct Registers {
    pc: ProgramCounter,
    delay: u8,
    sound: u8,
    sp: u8,
    i_reg: u16,
    v_regs: [u8; 16],
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

    fn load_rom(&mut self, file: &str) -> Result<(), String>{
        match File::open(file) {
            Ok(mut rom) => {
                let mut raw_bytes = Vec::new();
                rom.read_to_end(&mut raw_bytes)
                    .expect("Something went wrong while reading the rom");
                self.0[0x200..0x200 + raw_bytes.len()]
                    .copy_from_slice(&raw_bytes);
                Ok(())
            },
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
struct Screen([[bool; SCREEN_WIDTH]; SCREEN_HEIGHT]);

impl fmt::Debug for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut screen_string = "".to_string();
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                if self.0[y][x] {
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
            0: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
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
        for byte_num in 0..sprite.len() {
            for bit in 0..8 {
                let pixel_val = get_bit(sprite[byte_num], bit)
                    .expect("Iterator went over 8");
                let y_cord = Some((y as usize + byte_num) % (SCREEN_HEIGHT));
                let x_cord = Some((x + bit) as usize % (SCREEN_WIDTH));
                *collision_flag |= (pixel_val
                    & self.0[y_cord.expect("Should've gotten an x value")]
                        [x_cord.expect("Should've gotten a y value")])
                    as u8;

                self.0[y_cord.expect("Should've gotten an x value")]
                    [x_cord.expect("Should've gotten a y value")] ^= pixel_val
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct Keyboard {
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

    fn press_key(&mut self, key: Key, vreg: &mut [u8; 16]) {
        if let Some(key) = key_to_usize(key) {
            self.key_buffer[key] = true;
        }
        if self.wait_press != None {
            match key_to_usize(key) {
                Some(key) => {
                    vreg[self.wait_press.unwrap() as usize] = key as u8
                }
                None => (),
            }
            self.wait_press = None;
        }
    }

    fn release_key(&mut self, key: Key) {
        if let Some(key) = key_to_usize(key) {
            self.key_buffer[key] = false;
        }
    }
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

fn main() {
    let mut ram = match env::args().nth(1) {
        Some(path) => match Ram::initialize_ram(&path) {
            Ok(r) => r,
            Err(e) => {println!("{}", e); return},
        }
        _ => {println!("Didn't recieve a rom"); return},
    };

    let mut regs: Registers = Registers::new();
    let mut stack: Stack = Stack::new();
    let mut screen: Screen = Screen::new();
    let mut keyboard: Keyboard = Keyboard::new();
    let mut draw_flag: bool = false;

    let mut window: PistonWindow = WindowSettings::new(
        "Rust-8 Emulator",
        [SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32],
    ).exit_on_esc(true)
        .build()
        .unwrap();
    window.set_ups(60);

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            if draw_flag {
                draw_pixel_buffer(&screen);
                println!("{:?}", screen);
                draw_flag = false;
            }
        }

        if let Some(up_args) = e.update_args() {
            match emulate_cycles(
                up_args.dt,
                &mut ram,
                &mut regs,
                &mut stack,
                &mut screen,
                &mut keyboard,
                &mut draw_flag,
            ) {
                Ok(_) => (),
                Err(err) => {
                    println!("{:?}", err);
                    return ();
                }
            }
        }

        if let Some(k) = e.press_args() {
            match k {
                Button::Keyboard(input) => {
                    keyboard.press_key(input, &mut regs.v_regs)
                }
                _ => (),
            }
        }

        if let Some(k) = e.release_args() {
            match k {
                Button::Keyboard(input) => keyboard.release_key(input),
                _ => (),
            }
        }
    }
}

fn emulate_cycles(
    dt: f64,
    ram: &mut Ram,
    regs: &mut Registers,
    stack: &mut Stack,
    screen: &mut Screen,
    keyboard: &mut Keyboard,
    draw_flag: &mut bool,
) -> Result<(), InvalidOpcode> {
    if keyboard.wait_press == None {
        if regs.delay != 0 {
            regs.delay -= 1;
        }
        if regs.sound != 0 {
            regs.sound -= 1;
        }
        let num_inst = (dt * CLOCK_SPEED).round() as usize;

        for _ in 0..num_inst {
            if keyboard.wait_press == None {
                let op = Opcode::decode_op(fetch_opcode(&regs.pc, &ram))?;
                match execute(op, ram, regs, stack, screen, keyboard, draw_flag)
                {
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

fn draw_pixel_buffer(screen: &Screen) {}

fn fetch_opcode(pc: &ProgramCounter, ram: &Ram) -> u16 {
    let l_byte: u8 = ram.0[pc.get_addr() as usize];
    let r_byte: u8 = ram.0[(pc.get_addr() + 1) as usize];
    ((l_byte as u16) << 8) | (r_byte as u16)
}

fn execute(
    opcode: Opcode,
    ram: &mut Ram,
    regs: &mut Registers,
    stack: &mut Stack,
    screen: &mut Screen,
    keyboard: &mut Keyboard,
    draw_flag: &mut bool,
) -> Result<(), InvalidOpcode> {
    //println!("{:?}", regs);
    //println!("{:?}\n", opcode);
    match opcode {
        Opcode::NoArg(NoArg::ClearScreen) => {
            screen.0.iter_mut().for_each(|inner_array| {
                inner_array.iter_mut().for_each(|pixel| *pixel = false)
            });
            regs.pc.update();
            Ok(())
        }
        Opcode::NoArg(NoArg::ReturnSubrt) => match stack.pop(&mut regs.sp) {
            Ok(pc) => {
                regs.pc = pc;
                regs.pc.update();
                Ok(())
            }
            Err(err) => Err(InvalidOpcode::StackUnderflow(err, opcode)),
        },

        Opcode::OneArg(OneArg::SkipIfVx(arg)) => {
            skip_if_vx(arg, keyboard, &mut regs.pc);
            regs.pc.update();
            Ok(())
        }
        Opcode::OneArg(OneArg::SkipIfNVx(arg)) => {
            skip_if_not_vx(arg, keyboard, &mut regs.pc);
            regs.pc.update();
            Ok(())
        }
        Opcode::OneArg(OneArg::SetVxDT(arg)) => {
            regs.v_regs[arg.to_usize().expect("Check usize")] = regs.delay;
            regs.pc.update();
            Ok(())
        }
        Opcode::OneArg(OneArg::WaitForKey(arg)) => {
            keyboard.wait_press = Some(arg.to_u8().expect("Check u8"));
            regs.pc.update();
            Ok(())
        }
        Opcode::OneArg(OneArg::SetDT(arg)) => {
            regs.delay = regs.v_regs[arg.to_usize().expect("Check usize")];
            regs.pc.update();
            Ok(())
        }
        Opcode::OneArg(OneArg::SetST(arg)) => {
            regs.sound = regs.v_regs[arg.to_usize().expect("Check usize")];
            regs.pc.update();
            Ok(())
        }
        Opcode::OneArg(OneArg::SetI(arg)) => {
            regs.i_reg +=
                (regs.v_regs[arg.to_usize().expect("Check usize")]) as u16;
            regs.pc.update();

            Ok(())
        }
        Opcode::OneArg(OneArg::SetSpriteI(arg)) => match i_eq_spr_digit_vx(
            regs.v_regs[arg.to_usize().expect("Check usize")],
            &mut regs.i_reg,
        ) {
            Ok(_) => {
                regs.pc.update();
                Ok(())
            }
            Err(err) => Err(InvalidOpcode::NoSuchDigitSprite(err, opcode)),
        },
        Opcode::OneArg(OneArg::StoreDecVx(arg)) => {
            store_dec_vx_in_i(
                ram,
                regs.i_reg,
                regs.v_regs[arg.to_usize().expect("Check usize")],
            );
            regs.pc.update();
            Ok(())
        }
        Opcode::OneArg(OneArg::StoreV0Vx(arg)) => {
            store_v0_vx_in_ram(arg, ram, &mut regs.v_regs, &regs.i_reg);
            regs.pc.update();
            Ok(())
        }
        Opcode::OneArg(OneArg::ReadV0Vx(arg)) => {
            read_from_ram_in_v0_vx(arg, ram, &mut regs.v_regs, &regs.i_reg);
            regs.pc.update();
            Ok(())
        }
        Opcode::TwoArg(TwoArg::SkipEqVxVy(arg)) => {
            skip_vx_eq_vy(
                regs.v_regs[arg.x().to_usize().expect("Check usize")],
                regs.v_regs[arg.y().to_usize().expect("Check usize")],
                &mut regs.pc,
            );
            regs.pc.update();
            Ok(())
        }
        Opcode::TwoArg(TwoArg::VxEqVy(arg)) => {
            regs.v_regs[arg.x().to_usize().expect("Check usize")] =
                regs.v_regs[arg.y().to_usize().expect("Check usize")];
            regs.pc.update();
            Ok(())
        }
        Opcode::TwoArg(TwoArg::VxOREqVy(arg)) => {
            regs.v_regs[arg.x().to_usize().expect("Check usize")] |=
                regs.v_regs[arg.y().to_usize().expect("Check usize")];
            regs.pc.update();
            Ok(())
        }
        Opcode::TwoArg(TwoArg::VxANDEqVy(arg)) => {
            regs.v_regs[arg.x().to_usize().expect("Check usize")] &=
                regs.v_regs[arg.y().to_usize().expect("Check usize")];
            regs.pc.update();
            Ok(())
        }
        Opcode::TwoArg(TwoArg::VxXOREqVy(arg)) => {
            regs.v_regs[arg.x().to_usize().expect("Check usize")] ^=
                regs.v_regs[arg.y().to_usize().expect("Check usize")];
            regs.pc.update();
            Ok(())
        }
        Opcode::TwoArg(TwoArg::VxPlusEqVySetF(arg)) => {
            let (x, flag) = regs.v_regs
                [arg.x().to_usize().expect("Check usize")]
                .overflowing_add(
                    regs.v_regs[arg.y().to_usize().expect("Check usize")],
                );
            regs.v_regs[FLAG_REG] = flag as u8;
            regs.v_regs[arg.x().to_usize().expect("Check usize")] = x;
            regs.pc.update();
            Ok(())
        }
        Opcode::TwoArg(TwoArg::VxSubEqVySetF(arg)) => {
            let (x, flag) = regs.v_regs
                [arg.x().to_usize().expect("Check usize")]
                .overflowing_sub(
                    regs.v_regs[arg.y().to_usize().expect("Check usize")],
                );
            regs.v_regs[FLAG_REG] = (!flag) as u8;
            regs.v_regs[arg.x().to_usize().expect("Check usize")] = x;
            regs.pc.update();
            Ok(())
        }
        Opcode::TwoArg(TwoArg::ShiftVxR(arg)) => {
            regs.v_regs[FLAG_REG] = (0b00000001
                & regs.v_regs[arg.x().to_usize().expect("Check usize")]
                == 0b00000001) as u8;
            regs.v_regs[arg.x().to_usize().expect("Check usize")] =
                regs.v_regs[arg.x().to_usize().expect("Check usize")] >> 1;
            regs.pc.update();
            Ok(())
        }
        Opcode::TwoArg(TwoArg::VxEqVySubVxSetF(arg)) => {
            let (y, flag) = regs.v_regs
                [arg.y().to_usize().expect("Check usize")]
                .overflowing_sub(
                    regs.v_regs[arg.x().to_usize().expect("Check usize")],
                );
            regs.v_regs[FLAG_REG] = (!flag) as u8;
            regs.v_regs[arg.x().to_usize().expect("Check usize")] = y;
            regs.pc.update();
            Ok(())
        }
        Opcode::TwoArg(TwoArg::ShiftVxL(arg)) => {
            regs.v_regs[FLAG_REG] = (0b10000000
                & regs.v_regs[arg.x().to_usize().expect("Check usize")]
                ) >> 7 as u8;
            regs.v_regs[arg.x().to_usize().expect("Check usize")] =
                regs.v_regs[arg.x().to_usize().expect("Check usize")] << 1;
            regs.pc.update();
            Ok(())
        }
        Opcode::TwoArg(TwoArg::SkipVxNEqVy(arg)) => {
            skip_vx_neq_vy(
                regs.v_regs[arg.x().to_usize().expect("Check usize")],
                regs.v_regs[arg.y().to_usize().expect("Check usize")],
                &mut regs.pc,
            );
            regs.pc.update();
            Ok(())
        }
        Opcode::ThreeArg(ThreeArg::JumpToCodeRout(_)) => {
            regs.pc.update();
            Ok(())
        }
        Opcode::ThreeArg(ThreeArg::JumpToAddr(arg)) => {
            regs.pc.set_addr(arg.to_addr());
            Ok(())
        }
        Opcode::ThreeArg(ThreeArg::CallSubAt(arg)) => {
            match stack.push(&mut regs.sp, &regs.pc) {
                Ok(_) => {
                    regs.pc.set_addr(arg.to_addr());
                    Ok(())
                }
                Err(err) => Err(InvalidOpcode::StackOverflow(err, opcode)),
            }
        }
        Opcode::ThreeArg(ThreeArg::SkipVxEqKK(arg)) => {
            skip_vx_eq_kk(
                regs.v_regs[arg.x().to_usize().expect("Check usize")],
                arg.get_byte(),
                &mut regs.pc,
            );
            regs.pc.update();
            Ok(())
        }
        Opcode::ThreeArg(ThreeArg::SkipVxNEqKK(arg)) => {
            skip_vx_neq_kk(
                regs.v_regs[arg.x().to_usize().expect("Check usize")],
                arg.get_byte(),
                &mut regs.pc,
            );
            regs.pc.update();
            Ok(())
        }

        Opcode::ThreeArg(ThreeArg::SetVxKK(arg)) => {
            regs.v_regs[arg.x().to_usize().expect("Check usize") as usize] =
                arg.get_byte();
            regs.pc.update();
            Ok(())
        }
        Opcode::ThreeArg(ThreeArg::VxEqVxPlusKK(arg)) => {
            regs.v_regs[arg.x().to_usize().expect("Check usize")] = regs.v_regs
                [arg.x().to_usize().expect("Check usize")]
                .overflowing_add(arg.get_byte())
                .0;
            regs.pc.update();
            Ok(())
        }
        Opcode::ThreeArg(ThreeArg::SetIToNNN(arg)) => {
            regs.i_reg = arg.to_addr();
            regs.pc.update();
            Ok(())
        }
        Opcode::ThreeArg(ThreeArg::PCEqNNNPlusV0(arg)) => {
            let sum = (regs.v_regs[0] as usize) + arg.to_addr() as usize;
            if sum > 0xffe || sum < 0x200 {
                return Err(InvalidOpcode::OutOfBoundsAddress(
                    "Out of bounds program counter".to_string(),
                    opcode,
                ));
            }
            regs.pc.set_addr(sum as u16);
            Ok(())
        }
        Opcode::ThreeArg(ThreeArg::VxEqRandANDKK(arg)) => {
            let res = arg.get_byte() as usize & rand::random::<u8>() as usize;
            if res > 255 {
                return Err(InvalidOpcode::UndefBehavior(
                    "Overflow on random and".to_string(),
                    opcode,
                ));
            }
            regs.v_regs[arg.x().to_usize().expect("Check usize")] = res as u8;
            regs.pc.update();
            Ok(())
        }
        Opcode::ThreeArg(ThreeArg::DrawVxVyNib(arg)) => match screen
            .draw_nybble(
                regs.v_regs[arg.x().to_usize().expect("Check usize")],
                regs.v_regs[arg.y().to_usize().expect("Check usize")],
                regs.i_reg,
                Nybble::new([arg.last_nybble()]),
                &mut regs.v_regs[FLAG_REG],
                ram,
            ) {
            Ok(_) => {
                *draw_flag = true;
                regs.pc.update();
                Ok(())
            }
            Err(err) => Err(InvalidOpcode::OutOfScreenBounds(err, opcode)),
        },
    }
}

fn skip_vx_eq_kk(v_x: u8, byte: u8, pc: &mut ProgramCounter) {
    if v_x == byte {
        pc.update();
    }
}

fn skip_vx_neq_kk(v_x: u8, byte: u8, pc: &mut ProgramCounter) {
    if v_x != byte {
        pc.update();
    }
}

fn skip_vx_eq_vy(v_x: u8, v_y: u8, pc: &mut ProgramCounter) {
    if v_y == v_x {
        pc.update();
    }
}

// Note: Instructioins 8xyE and 8xy6 change depending on the interpreter.
// Double check for odd  emulator behaviour

fn skip_vx_neq_vy(v_x: u8, v_y: u8, pc: &mut ProgramCounter) {
    if v_x != v_y {
        pc.update();
    }
}

fn skip_if_vx(byte_arg: Nybble, keyboard: &Keyboard, pc: &mut ProgramCounter) {
    if keyboard.key_buffer[byte_arg.to_usize().expect("Check usize")] {
        pc.update();
    }
}

fn skip_if_not_vx(
    byte_arg: Nybble,
    keyboard: &Keyboard,
    pc: &mut ProgramCounter,
) {
    if !keyboard.key_buffer[byte_arg.to_usize().expect("Check usize")] {
        pc.update();
    }
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

fn store_dec_vx_in_i(ram: &mut Ram, i_reg: u16, v_reg: u8) {
    ram.0[i_reg as usize] = v_reg / 100;
    ram.0[(i_reg + 1) as usize] = (v_reg % 100) / 10;
    ram.0[(i_reg + 2) as usize] = v_reg % 10;
}

fn read_from_ram_in_v0_vx(
    byte_arg: Nybble,
    ram: &mut Ram,
    v_regs: &mut [u8; 16],
    i_reg: &u16,
) {
    for index in 0..=byte_arg.to_usize().expect("Check usize") {
        v_regs[index as usize] = ram.0[(*i_reg + index as u16) as usize];
    }
}

fn store_v0_vx_in_ram(
    byte_arg: Nybble,
    ram: &mut Ram,
    v_regs: &mut [u8; 16],
    i_reg: &u16,
) {
    for index in 0..=byte_arg.to_u64().expect("Check to_u64") {
        ram.0[(*i_reg + index as u16) as usize] = v_regs[index as usize]
    }
}

// Note, this will panic if you attempt to pass in a value over 7, as it is
// "out of bounds" for indexing a u8.

fn get_bit(n: u8, b: u8) -> Result<bool, String> {
    if b > 7 {
        return Err(format!("Attempted to pass in a val greater than 7 {}", b));
    }
    Ok((n >> (7 - b)) & 1 == 1)
}

#[test]
fn test_fetch_opcode() {
    let mut ram: Ram = Ram::new();
    let mut regs: Registers = Registers::new();
    regs.pc.set_addr(0);
    ram.0[0] = 0xFF;
    ram.0[1] = 0xA2;
    assert_eq!(fetch_opcode(&regs.pc, &ram), 0xFFA2);
}

#[test]
fn test_to_addr() {
    assert_eq!(0x0FBA, ThreeNybbles::new([0x0F, 0xBA]).to_addr())
}

#[test]
#[should_panic]
fn test_nybble() {
    let nybble: Nybble = Nybble::new([0xFA]);
}

#[test]
#[should_panic]
fn test_triple_nybble() {
    let tnybble: ThreeNybbles = ThreeNybbles::new([0xFA, 0xFD]);
}

#[test]
#[should_panic]
fn test_decode_op() {
    let chip8_addr: u16 = 0x200;
    let amount_of_ops: u16 = 35;
    let mut ram: Ram = Ram::new();
    let mut regs: Registers = Registers::new();
    let test_ops: [u8; 70] = [
        0x05, 0x55, 0x00, 0xE0, 0x00, 0xEE, 0x15, 0x55, 0x25, 0x55, 0x31, 0x33,
        0x41, 0x33, 0x56, 0x70, 0x61, 0x33, 0x71, 0x33, 0x86, 0x70, 0x86, 0x71,
        0x86, 0x72, 0x86, 0x73, 0x86, 0x74, 0x86, 0x75, 0x86, 0x76, 0x86, 0x77,
        0x86, 0x7E, 0x96, 0x70, 0xA5, 0x55, 0xB5, 0x55, 0xC1, 0x33, 0xD6, 0x75,
        0xE1, 0x9E, 0xE1, 0xA1, 0xF1, 0x07, 0xF1, 0x0A, 0xF1, 0x15, 0xF1, 0x18,
        0xF1, 0x1E, 0xF1, 0x29, 0xF1, 0x33, 0xF1, 0x55, 0xF1, 0x65,
    ];
    let mut x = chip8_addr;
    for element in test_ops.iter() {
        ram.0[x as usize] = *element;
        x += 1;
    }
    loop {
        let op = Opcode::decode_op(fetch_opcode(&regs.pc, &ram));
        regs.pc.update();
        if (regs.pc.get_addr() == (chip8_addr + (amount_of_ops * 2))) {
            break;
        }
    }
}

#[test]
fn test_rom_loader() {
    panic!();
}

#[test]
#[should_panic]
fn test_wrong_rom_path() {}

#[test]
fn test_rom_ram_loader() {
    panic!();
}
