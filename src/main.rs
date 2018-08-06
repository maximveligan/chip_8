extern crate num;
extern crate rand;
use num::ToPrimitive;
use std::fmt;
use nybble::Nybble;
use nybble::ThreeNybbles;
use nybble::TwoNybbles;
use std::fs::File;
use std::io::prelude::*;

mod nybble;

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
const SPR_F_START: u16 = 0075;

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

const CLEAR_SCREEN: u16 = 0x00E0;
const RET_SUBROUTINE: u16 = 0x00EE;
const SKIP_IF_VX: u16 = 0xE09E;
const SKIP_IF_NOT_VX: u16 = 0xE0A1;
const SET_VX_DT: u16 = 0xF007;
const WAIT_FOR_KEY: u16 = 0xF00A;
const SET_DT: u16 = 0xF015;
const SET_ST: u16 = 0xF018;
const SET_I: u16 = 0xF01E;
const SET_SPR_I: u16 = 0xF029;
const STORE_DEC_VX: u16 = 0xF033;
const STORE_V0_VX: u16 = 0xF055;
const READ_V0_VX: u16 = 0xF065;
const SKIP_VX_EQ_VY: u16 = 0x5000;
const VX_EQ_VY: u16 = 0x8000;
const VX_OR_EQ_VY: u16 = 0x8001;
const VX_AND_EQ_VY: u16 = 0x8002;
const VX_XOR_EQ_VY: u16 = 0x8003;
const VX_PLUS_EQ_VY_F: u16 = 0x8004;
const VX_SUB_EQ_VY_F: u16 = 0x8005;
const SHIFT_VX_R: u16 = 0x8006;
const VX_EQ_VY_SUB_VX_F: u16 = 0x8007;
const SHIFT_VX_L: u16 = 0x800E;
const SKIP_VX_NOT_VY: u16 = 0x9000;
const JUMP_TO_CODEROUTE: u16 = 0x0000;
const JUMP_TO_ADDR: u16 = 0x1000;
const CALL_SUB_AT_ADDR: u16 = 0x2000;
const SKIP_VX_EQ_KK: u16 = 0x3000;
const SKIP_VX_NEQ_KK: u16 = 0x4000;
const VX_EQ_KK: u16 = 0x6000;
const VX_PLUS_EQ_KK: u16 = 0x7000;
const I_EQ_NNN: u16 = 0xA000;
const PC_EQ_V0_PLUS_NNN: u16 = 0xB000;
const VX_EQ_RAND_PLUS_KK: u16 = 0xC000;
const DRAW_VX_VY_NIB: u16 = 0xD000;

#[derive(Debug, Clone)]
enum Opcode {
    NoArg(NoArg),
    OneArg(OneArg),
    TwoArg(TwoArg),
    ThreeArg(ThreeArg),
}

#[derive(Debug, Copy, Clone)]
enum NoArg {
    ClearScreen, //00E0
    ReturnSubrt, //00EE
}

#[derive(Debug, Clone)]
enum OneArg {
    SkipIfVx(Nybble),   //Ex9E
    SkipIfNVx(Nybble),  //ExA1
    SetVxDT(Nybble),    //Fx07
    WaitForKey(Nybble), //Fx0A
    SetDT(Nybble),      //Fx15
    SetST(Nybble),      //Fx18
    SetI(Nybble),       //Fx1E
    SetSpriteI(Nybble), //Fx29
    StoreDecVx(Nybble), //Fx33
    StoreV0Vx(Nybble),  //Fx55
    ReadV0Vx(Nybble),   //Fx65
}

#[derive(Debug, Clone)]
enum TwoArg {
    SkipEqVxVy(TwoNybbles),      // 5xy0
    VxEqVy(TwoNybbles),          //8xy0
    VxOREqVy(TwoNybbles),        //8xy1
    VxANDEqVy(TwoNybbles),       //8xy2
    VxXOREqVy(TwoNybbles),       //8xy3
    VxPlusEqVySetF(TwoNybbles),  //8xy4
    VxSubEqVySetF(TwoNybbles),   //8xy5
    ShiftVxR(TwoNybbles),        //8xy6
    VxEqVySubVxSetF(TwoNybbles), //8xy7
    ShiftVxL(TwoNybbles),        //8xyE
    SkipVxNEqVy(TwoNybbles),     //9xy0
}

#[derive(Debug, Clone)]
enum ThreeArg {
    JumpToCodeRout(ThreeNybbles), //0nnn
    JumpToAddr(ThreeNybbles),     //1nnn
    CallSubAt(ThreeNybbles),      //2nnn
    SkipVxEqKK(ThreeNybbles),     //3xkk
    SkipVxNEqKK(ThreeNybbles),    //4xkk
    SetVxKK(ThreeNybbles),        //6xkk
    VxEqVxPlusKK(ThreeNybbles),   //7xkk
    SetIToNNN(ThreeNybbles),      //Annn
    PCEqNNNPlusV0(ThreeNybbles),  //Bnnn
    VxEqRandANDKK(ThreeNybbles),  //Cxkk
    DrawVxVyNib(ThreeNybbles),    //Dxyn
}

#[derive(Debug, Clone)]
enum InvalidOpcode {
    DoesntExist(String),
    StackOverflow(String, Stack, Opcode, ProgramCounter, Ram),
    StackUnderflow(String, Stack, Opcode, ProgramCounter, Ram),
    OutOfBoundsAddress(String, Opcode, ProgramCounter, Ram),
    UnevenAddress(String, Opcode, ProgramCounter, Ram),
    NoSuchDigitSprite(String, Opcode, ProgramCounter, Ram),
    OutOfScreenBounds(String, Opcode, ProgramCounter, Ram, Screen),
    UndefBehavior(String, Opcode, ProgramCounter, Ram),
}

#[derive(Debug)]
struct Argument(u16);

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
    fn new() -> Ram {
        Ram { 0: [0; 0xFFF] }
    }
    fn load_digit_data(&mut self) {
        self.0[0000..0004].copy_from_slice(&SPR_ZERO);
        self.0[0005..0009].copy_from_slice(&SPR_ONE);
        self.0[0010..0014].copy_from_slice(&SPR_TWO);
        self.0[0015..0019].copy_from_slice(&SPR_THREE);
        self.0[0020..0024].copy_from_slice(&SPR_FOUR);
        self.0[0025..0029].copy_from_slice(&SPR_FIVE);
        self.0[0030..0034].copy_from_slice(&SPR_SIX);
        self.0[0035..0039].copy_from_slice(&SPR_SEVEN);
        self.0[0040..0044].copy_from_slice(&SPR_EIGHT);
        self.0[0045..0049].copy_from_slice(&SPR_NINE);
        self.0[0050..0054].copy_from_slice(&SPR_A);
        self.0[0055..0059].copy_from_slice(&SPR_B);
        self.0[0060..0064].copy_from_slice(&SPR_C);
        self.0[0065..0069].copy_from_slice(&SPR_D);
        self.0[0070..0074].copy_from_slice(&SPR_E);
        self.0[0075..0079].copy_from_slice(&SPR_F);
    }

    //  This might be wrong, double check the logic

    fn retrieve_bytes(self, index: u16, amount: Nybble) -> Vec<u8> {
        let mut byte_vec = Vec::new();
        for byte in self.0[index as usize]
            ..self.0[amount.to_usize().expect("Check usize")]
        {
            byte_vec.push(byte);
        }
        byte_vec
    }
}

#[derive(Debug, Clone, Copy)]
struct ProgramCounter(u16);

impl ProgramCounter {
    fn update(&mut self) {
        self.0 += 2;
    }
}

#[derive(Debug, Clone, Copy)]
struct Stack([u16; 16]);

impl Stack {
    fn new() -> Stack {
        Stack { 0: [0; 16] }
    }
    fn push(&mut self, sp: &mut u8, pc: &ProgramCounter) -> Result<(), String> {
        if *sp >= 15 {
            return Err("Stack overflow".to_string());
        }
        *sp = *sp + 1;
        self.0[*sp as usize] = pc.0;
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
        write!(f, "Pixel Buffer Dump\n{}", screen_string)
    }
}

impl Screen {
    fn new() -> Screen {
        Screen {
            0: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Keyboard([bool; 0xF + 1]);

impl Keyboard {
    fn new() -> Keyboard {
        Keyboard {
            0: [false; 0xF + 1],
        }
    }
}

fn main() {
    let mut ram: Ram = Ram::new();
    let mut regs: Registers = Registers::new();
    let mut stack: Stack = Stack::new();
    let mut screen: Screen = Screen::new();
    let keyboard: Keyboard = Keyboard::new();
    loop {
        match decode_op(fetch_opcode(&regs.pc, &ram)) {
            Ok(op) => execute(
                op,
                &mut ram,
                &mut regs,
                &mut stack,
                &mut screen,
                &keyboard,
            ),
            Err(inv_op) => panic!("{:?}", inv_op),
        };
        regs.pc.update();
    }
}

fn load_rom(file: &str, ram: &mut Ram) {
    let mut rom = File::open(file).expect("Rom not found");
    let mut raw_bytes = Vec::new();
    rom.read_to_end(&mut raw_bytes)
        .expect("Something went wrong while reading the rom");
    &ram.0[0x200..0xFFF].copy_from_slice(&raw_bytes);
}

fn fetch_opcode(pc: &ProgramCounter, ram: &Ram) -> u16 {
    let l_byte: u8 = ram.0[pc.0 as usize];
    let r_byte: u8 = ram.0[(pc.0 + 1) as usize];
    ((l_byte as u16) << 8) | (r_byte as u16)
}

// TODO: Run tests on games and see which opcodes appear most frequently.
// Reorder this table so that the frequent ones are higher in order

fn decode_op(op: u16) -> Result<Opcode, InvalidOpcode> {
    match op {
        CLEAR_SCREEN => Ok(Opcode::NoArg(NoArg::ClearScreen)),
        RET_SUBROUTINE => Ok(Opcode::NoArg(NoArg::ReturnSubrt)),
        op if ((op & READ_V0_VX) == READ_V0_VX) => {
            Ok(Opcode::OneArg(OneArg::ReadV0Vx(Nybble::from(op))))
        }
        op if ((op & STORE_V0_VX) == STORE_V0_VX) => {
            Ok(Opcode::OneArg(OneArg::StoreV0Vx(Nybble::from(op))))
        }
        op if ((op & STORE_DEC_VX) == STORE_DEC_VX) => {
            Ok(Opcode::OneArg(OneArg::StoreDecVx(Nybble::from(op))))
        }
        op if ((op & SET_SPR_I) == SET_SPR_I) => {
            Ok(Opcode::OneArg(OneArg::SetSpriteI(Nybble::from(op))))
        }
        op if ((op & SET_I) == SET_I) => {
            Ok(Opcode::OneArg(OneArg::SetI(Nybble::from(op))))
        }
        op if ((op & SET_ST) == SET_ST) => {
            Ok(Opcode::OneArg(OneArg::SetST(Nybble::from(op))))
        }
        op if ((op & SET_DT) == SET_DT) => {
            Ok(Opcode::OneArg(OneArg::SetDT(Nybble::from(op))))
        }
        op if ((op & WAIT_FOR_KEY) == WAIT_FOR_KEY) => {
            Ok(Opcode::OneArg(OneArg::WaitForKey(Nybble::from(op))))
        }
        op if ((op & SET_VX_DT) == SET_VX_DT) => {
            Ok(Opcode::OneArg(OneArg::SetVxDT(Nybble::from(op))))
        }
        op if ((op & VX_PLUS_EQ_KK) == VX_PLUS_EQ_KK) => Ok(Opcode::ThreeArg(
            ThreeArg::VxEqVxPlusKK(ThreeNybbles::from(op)),
        )),
        op if ((op & VX_EQ_KK) == VX_EQ_KK) => {
            Ok(Opcode::ThreeArg(ThreeArg::SetVxKK(ThreeNybbles::from(op))))
        }
        op if ((op & SKIP_VX_EQ_VY) == SKIP_VX_EQ_VY) => {
            Ok(Opcode::TwoArg(TwoArg::SkipEqVxVy(TwoNybbles::from(op))))
        }
        op if ((op & SKIP_VX_NEQ_KK) == SKIP_VX_NEQ_KK) => Ok(
            Opcode::ThreeArg(ThreeArg::SkipVxNEqKK(ThreeNybbles::from(op))),
        ),
        op if ((op & SKIP_VX_EQ_KK) == SKIP_VX_EQ_KK) => Ok(Opcode::ThreeArg(
            ThreeArg::SkipVxEqKK(ThreeNybbles::from(op)),
        )),
        op if ((op & CALL_SUB_AT_ADDR) == CALL_SUB_AT_ADDR) => Ok(
            Opcode::ThreeArg(ThreeArg::CallSubAt(ThreeNybbles::from(op))),
        ),
        op if ((op & JUMP_TO_ADDR) == JUMP_TO_ADDR) => Ok(Opcode::ThreeArg(
            ThreeArg::JumpToAddr(ThreeNybbles::from(op)),
        )),
        op if ((op & JUMP_TO_CODEROUTE) == JUMP_TO_CODEROUTE) => Ok(
            Opcode::ThreeArg(ThreeArg::JumpToCodeRout(ThreeNybbles::from(op))),
        ),
        op if ((op & SKIP_IF_NOT_VX) == SKIP_IF_NOT_VX) => {
            Ok(Opcode::OneArg(OneArg::SkipIfNVx(Nybble::from(op))))
        }
        op if ((op & SKIP_IF_VX) == SKIP_IF_VX) => {
            Ok(Opcode::OneArg(OneArg::SkipIfVx(Nybble::from(op))))
        }
        op if ((op & DRAW_VX_VY_NIB) == DRAW_VX_VY_NIB) => Ok(
            Opcode::ThreeArg(ThreeArg::DrawVxVyNib(ThreeNybbles::from(op))),
        ),
        op if ((op & VX_EQ_RAND_PLUS_KK) == VX_EQ_RAND_PLUS_KK) => Ok(
            Opcode::ThreeArg(ThreeArg::VxEqRandANDKK(ThreeNybbles::from(op))),
        ),
        op if ((op & PC_EQ_V0_PLUS_NNN) == PC_EQ_V0_PLUS_NNN) => Ok(
            Opcode::ThreeArg(ThreeArg::PCEqNNNPlusV0(ThreeNybbles::from(op))),
        ),
        op if ((op & I_EQ_NNN) == I_EQ_NNN) => Ok(Opcode::ThreeArg(
            ThreeArg::SetIToNNN(ThreeNybbles::from(op)),
        )),
        op if ((op & SKIP_VX_NOT_VY) == SKIP_VX_NOT_VY) => {
            Ok(Opcode::TwoArg(TwoArg::SkipVxNEqVy(TwoNybbles::from(op))))
        }
        op if ((op & SHIFT_VX_L) == SHIFT_VX_L) => {
            Ok(Opcode::TwoArg(TwoArg::ShiftVxL(TwoNybbles::from(op))))
        }
        op if ((op & VX_EQ_VY_SUB_VX_F) == VX_EQ_VY_SUB_VX_F) => Ok(
            Opcode::TwoArg(TwoArg::VxEqVySubVxSetF(TwoNybbles::from(op))),
        ),
        op if ((op & SHIFT_VX_R) == SHIFT_VX_R) => {
            Ok(Opcode::TwoArg(TwoArg::ShiftVxR(TwoNybbles::from(op))))
        }
        op if ((op & VX_SUB_EQ_VY_F) == VX_SUB_EQ_VY_F) => {
            Ok(Opcode::TwoArg(TwoArg::VxSubEqVySetF(TwoNybbles::from(op))))
        }
        op if ((op & VX_PLUS_EQ_VY_F) == VX_PLUS_EQ_VY_F) => {
            Ok(Opcode::TwoArg(TwoArg::VxPlusEqVySetF(TwoNybbles::from(op))))
        }
        op if ((op & VX_XOR_EQ_VY) == VX_XOR_EQ_VY) => {
            Ok(Opcode::TwoArg(TwoArg::VxXOREqVy(TwoNybbles::from(op))))
        }
        op if ((op & VX_AND_EQ_VY) == VX_AND_EQ_VY) => {
            Ok(Opcode::TwoArg(TwoArg::VxANDEqVy(TwoNybbles::from(op))))
        }
        op if ((op & VX_OR_EQ_VY) == VX_OR_EQ_VY) => {
            Ok(Opcode::TwoArg(TwoArg::VxOREqVy(TwoNybbles::from(op))))
        }
        op if ((op & VX_EQ_VY) == VX_EQ_VY) => {
            Ok(Opcode::TwoArg(TwoArg::VxEqVy(TwoNybbles::from(op))))
        }
        _ => Err(InvalidOpcode::DoesntExist(format!(
            "Unsupported op {:X}",
            op
        ))),
    }
}

fn execute(
    opcode: Opcode,
    ram: &mut Ram,
    regs: &mut Registers,
    stack: &mut Stack,
    screen: &mut Screen,
    keyboard: &Keyboard,
) -> Result<(), InvalidOpcode> {
    match opcode {
        Opcode::NoArg(NoArg::ClearScreen) => {
            Ok(screen.0.iter_mut().for_each(|inner_array| {
                inner_array.iter_mut().for_each(|pixel| *pixel = false)
            }))
        }
        Opcode::NoArg(NoArg::ReturnSubrt) => match stack.pop(&mut regs.sp) {
            Ok(pc) => Ok(regs.pc = pc),

            Err(err) => Err(InvalidOpcode::StackUnderflow(
                err,
                stack.clone(),
                opcode,
                regs.pc,
                *ram,
            )),
        },

        Opcode::OneArg(OneArg::SkipIfVx(arg)) => {
            Ok(skip_if_vx(arg, keyboard, &mut regs.pc))
        }
        Opcode::OneArg(OneArg::SkipIfNVx(arg)) => {
            Ok(skip_if_not_vx(arg, keyboard, &mut regs.pc))
        }
        Opcode::OneArg(OneArg::SetVxDT(arg)) => {
            Ok(regs.v_regs[arg.to_usize().expect("Check usize")] = regs.delay)
        }
        Opcode::OneArg(OneArg::WaitForKey(arg)) => Ok(load_key_vx(arg)),
        Opcode::OneArg(OneArg::SetDT(arg)) => {
            Ok(regs.delay = regs.v_regs[arg.to_usize().expect("Check usize")])
        }
        Opcode::OneArg(OneArg::SetST(arg)) => {
            Ok(regs.sound = regs.v_regs[arg.to_usize().expect("Check usize")])
        }
        Opcode::OneArg(OneArg::SetI(arg)) => Ok(regs.i_reg +=
            (regs.v_regs[arg.to_usize().expect("Check usize")]) as u16),
        Opcode::OneArg(OneArg::SetSpriteI(arg)) => match i_eq_spr_digit_vx(
            regs.v_regs[arg.to_usize().expect("Check usize")],
            &mut regs.i_reg,
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(InvalidOpcode::NoSuchDigitSprite(
                err, opcode, regs.pc, *ram,
            )),
        },
        Opcode::OneArg(OneArg::StoreDecVx(arg)) => Ok(store_dec_vx_in_i(
            ram,
            regs.i_reg,
            regs.v_regs[arg.to_usize().expect("Check usize")],
        )),
        Opcode::OneArg(OneArg::StoreV0Vx(arg)) => {
            Ok(store_v0_vx_in_ram(arg, ram, &mut regs.v_regs, &regs.i_reg))
        }
        Opcode::OneArg(OneArg::ReadV0Vx(arg)) => Ok(read_from_ram_in_v0_vx(
            arg,
            ram,
            &mut regs.v_regs,
            &regs.i_reg,
        )),
        Opcode::TwoArg(TwoArg::SkipEqVxVy(arg)) => Ok(skip_vx_eq_vy(
            regs.v_regs[arg.x().to_usize().expect("Check usize")],
            regs.v_regs[arg.y().to_usize().expect("Check usize")],
            &mut regs.pc,
        )),
        Opcode::TwoArg(TwoArg::VxEqVy(arg)) => Ok(regs.v_regs
            [arg.x().to_usize().expect("Check usize")] =
            regs.v_regs[arg.y().to_usize().expect("Check usize")]),
        Opcode::TwoArg(TwoArg::VxOREqVy(arg)) => Ok(regs.v_regs
            [arg.x().to_usize().expect("Check usize")] |=
            regs.v_regs[arg.y().to_usize().expect("Check usize")]),
        Opcode::TwoArg(TwoArg::VxANDEqVy(arg)) => Ok(regs.v_regs
            [arg.x().to_usize().expect("Check usize")] &=
            regs.v_regs[arg.y().to_usize().expect("Check usize")]),
        Opcode::TwoArg(TwoArg::VxXOREqVy(arg)) => Ok(regs.v_regs
            [arg.x().to_usize().expect("Check usize")] ^=
            regs.v_regs[arg.y().to_usize().expect("Check usize")]),
        Opcode::TwoArg(TwoArg::VxPlusEqVySetF(arg)) => {
            let (x, flag) = regs.v_regs
                [arg.x().to_usize().expect("Check usize")]
                .overflowing_add(
                    regs.v_regs[arg.y().to_usize().expect("Check usize")],
                );
            regs.v_regs[FLAG_REG] = flag as u8;
            regs.v_regs[arg.x().to_usize().expect("Check usize")] = x;
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
            Ok(())
        }
        Opcode::TwoArg(TwoArg::ShiftVxR(arg)) => {
            regs.v_regs[FLAG_REG] = (0b00000001
                & regs.v_regs[arg.x().to_usize().expect("Check usize")]
                == 0b00000001) as u8;
            regs.v_regs[arg.x().to_usize().expect("Check usize")] >> 1;
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
            Ok(())
        }
        Opcode::TwoArg(TwoArg::ShiftVxL(arg)) => {
            regs.v_regs[FLAG_REG] = (0b1000000
                & regs.v_regs[arg.x().to_usize().expect("Check usize")]
                == 0b10000000) as u8;
            regs.v_regs[arg.x().to_usize().expect("Check usize")] << 1;
            Ok(())
        }
        Opcode::TwoArg(TwoArg::SkipVxNEqVy(arg)) => Ok(skip_vx_neq_vy(
            regs.v_regs[arg.x().to_usize().expect("Check usize")],
            regs.v_regs[arg.y().to_usize().expect("Check usize")],
            &mut regs.pc,
        )),
        Opcode::ThreeArg(ThreeArg::JumpToCodeRout(arg)) => Ok(()),
        Opcode::ThreeArg(ThreeArg::JumpToAddr(arg)) => {
            let addr = arg.to_addr();
            if addr % 2 != 0 {
                return Err(InvalidOpcode::UnevenAddress(
                    "Uneven address".to_string(),
                    opcode,
                    regs.pc,
                    *ram,
                ));
            }
            regs.pc.0 = arg.to_addr();
            Ok(())
        }
        Opcode::ThreeArg(ThreeArg::CallSubAt(arg)) => {
            match stack.push(&mut regs.sp, &regs.pc) {
                Ok(_) => Ok(regs.pc.0 = arg.to_addr()),
                Err(err) => Err(InvalidOpcode::StackOverflow(
                    err, *stack, opcode, regs.pc, *ram,
                )),
            }
        }
        Opcode::ThreeArg(ThreeArg::SkipVxEqKK(arg)) => Ok(skip_vx_eq_kk(
            regs.v_regs[arg.x().to_usize().expect("Check usize")],
            arg.get_byte(),
            &mut regs.pc,
        )),
        Opcode::ThreeArg(ThreeArg::SkipVxNEqKK(arg)) => Ok(skip_vx_neq_kk(
            regs.v_regs[arg.x().to_usize().expect("Check usize")],
            arg.get_byte(),
            &mut regs.pc,
        )),
        Opcode::ThreeArg(ThreeArg::SetVxKK(arg)) => Ok(regs.v_regs
            [arg.x().to_usize().expect("Check usize") as usize] =
            arg.get_byte()),
        Opcode::ThreeArg(ThreeArg::VxEqVxPlusKK(arg)) => {
            let sum: usize = arg.get_byte() as usize
                + regs.v_regs[arg.x().to_usize().expect("Check usize")]
                    as usize;
            if sum >= 256 {
                return Err(InvalidOpcode::UndefBehavior(
                    "Vx += KK overflowed".to_string(),
                    opcode,
                    regs.pc,
                    *ram,
                ));
            } else {
                Ok(regs.v_regs[arg.x().to_usize().expect("Check usize")] =
                    sum as u8)
            }
        }
        Opcode::ThreeArg(ThreeArg::SetIToNNN(arg)) => {
            Ok(regs.i_reg = arg.to_addr())
        }
        Opcode::ThreeArg(ThreeArg::PCEqNNNPlusV0(arg)) => {
            let sum = (regs.v_regs[0] as usize) + arg.to_addr() as usize;
            if sum % 2 != 0 {
                return Err(InvalidOpcode::UnevenAddress(
                    "Uneven address".to_string(),
                    opcode,
                    regs.pc,
                    *ram,
                ));
            }
            if sum > 0xffe || sum < 0x200 {
                return Err(InvalidOpcode::OutOfBoundsAddress(
                    "Out of bounds program counter".to_string(),
                    opcode,
                    regs.pc,
                    *ram,
                ));
            }
            Ok(regs.pc.0 = sum as u16)
        }
        Opcode::ThreeArg(ThreeArg::VxEqRandANDKK(arg)) => {
            let res = arg.get_byte() as usize & rand::random::<u8>() as usize;
            if res > 255 {
                return Err(InvalidOpcode::UndefBehavior(
                    "Overflow on random and".to_string(),
                    opcode,
                    regs.pc,
                    *ram,
                ));
            }
            Ok(regs.v_regs[arg.x().to_usize().expect("Check usize")] =
                res as u8)
        }
        Opcode::ThreeArg(ThreeArg::DrawVxVyNib(arg)) => {
            match draw_vx_vy_nybble(
                arg.get_byte(),
                regs.v_regs[arg.x().to_usize().expect("Check usize")],
                regs.v_regs[arg.y().to_usize().expect("Check usize")],
                regs.i_reg,
                &mut regs.v_regs[FLAG_REG],
                ram,
                screen,
            ) {
                Ok(_) => Ok(()),
                Err(err) => Err(InvalidOpcode::OutOfScreenBounds(
                    err, opcode, regs.pc, *ram, *screen,
                )),
            }
        }
    }
}

//  Possible optimization of next three, abstract into higher order function

fn skip_vx_eq_kk(v_x: u8, byte: u8, pc: &mut ProgramCounter) {
    if (v_x == byte) {
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

fn draw_vx_vy_nybble(
    byte_num: u8,
    v_x: u8,
    v_y: u8,
    i_reg: u16,
    flag: &mut u8,
    ram: &mut Ram,
    screen: &mut Screen,
) -> Result<(), String> {
    Ok(())
}

fn skip_if_vx(byte_arg: Nybble, keyboard: &Keyboard, pc: &mut ProgramCounter) {
    if keyboard.0[byte_arg.to_usize().expect("Check usize")] {
        pc.update();
    }
}

fn skip_if_not_vx(
    byte_arg: Nybble,
    keyboard: &Keyboard,
    pc: &mut ProgramCounter,
) {
    if !keyboard.0[byte_arg.to_usize().expect("Check usize")] {
        pc.update();
    }
}

fn load_key_vx(byte_arg: Nybble) {
    println!("Got to opcode {:?}", byte_arg);
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
    for index in 0..byte_arg.to_usize().expect("Check usize") {
        v_regs[index as usize] = ram.0[(*i_reg + index as u16) as usize];
    }
}

fn store_v0_vx_in_ram(
    byte_arg: Nybble,
    ram: &mut Ram,
    v_regs: &mut [u8; 16],
    i_reg: &u16,
) {
    for index in 0..byte_arg.to_u64().expect("Check to_u64") {
        ram.0[(*i_reg + index as u16) as usize] = v_regs[index as usize]
    }
}

// Note, this will panic if you attempt to pass in a value over 7, as it is
// "out of bounds" for indexing a u8.

fn get_bit(n: u8, b: u8) -> bool {
    ((n >> (7 - b)) & 1 == 1)
}

#[test]
fn test_fetch_opcode() {
    let mut ram: Ram = Ram::new();
    let mut regs: Registers = Registers::new();
    regs.pc.0 = 0;
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
        decode_op(fetch_opcode(&regs.pc, &ram))?;
        regs.pc.update();
        if (regs.pc.0 == (chip8_addr + (amount_of_ops * 2))) {
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
