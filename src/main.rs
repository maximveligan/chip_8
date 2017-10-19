extern crate rand;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct Interpreter {
    regs: Registers,
}

enum Opcode {
    ZeroArg(ZeroArg),
    OneArg(OneArg),
    TwoArg(TwoArg),
    ThreeArg(ThreeArg),
}

const CLEAR_SCREEN: u16 = 0x00E0;
const RET_SUBROUTINE: u16 = 0x00EE;
const SKIP_IF_VX: u16 = 0xE09E;
const SKIP_IF_NOT_VX: u16 = 0xE0A1;
const SET_VX_DT: u16 = 0xF007;
const WAIT_FOR_KEY: u16 = 0xF00A;
const SET_DT: u16 = 0xF015;
const SET_ST: u16 = 0xF018;
const SET_I: u16 = 0xF01E;
const SET_SPRITE_I: u16 = 0xF029;
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
const SKIP_IF_VX_NOT_VY: u16 = 0x9000;
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

enum ZeroArg {
    ClearScreen, //00E0
    ReturnSubrt, //00EE
}

enum OneArg {
    SkipIfVx(Nybble), //Ex9E
    SkipIfNotVx(Nybble), //ExA1
    SetVxDT(Nybble), //Fx07
    WaitForKey(Nybble), //Fx0A
    SetDT(Nybble), //Fx15
    SetST(Nybble), //Fx18
    SetI(Nybble), //Fx1E
    SetSpriteI(Nybble), //Fx29
    StoreDecVx(Nybble), //Fx33
    StoreV0Vx(Nybble), //Fx55
    ReadV0Vx(Nybble), //Fx65
}

enum TwoArg {
    SkipEqVxVy(DoubleNybble), // 5xy0
    VxEqVy(DoubleNybble), //8xy0
    VxEqVxORVy(DoubleNybble), //8xy1
    VxEqVxANDVy(DoubleNybble), //8xy2
    VxEqVxXORVy(DoubleNybble), //8xy3
    VxEqVxPlusVySetF(DoubleNybble), //8xy4
    VxEqVxSubVySetF(DoubleNybble), //8xy5
    ShiftVxRight(DoubleNybble), //8xy6
    VxEqVySubVxSetF(DoubleNybble), //8xy7
    ShiftVxLeft(DoubleNybble), //8xyE
    SkipIfVxNotEqVy(DoubleNybble), //9xy0
}

enum ThreeArg {
    JumpToCodeRoutNNN(TripleNybble), //0nnn
    JumpToAddrNNN(TripleNybble), //1nnn
    CallSubAtNNN(TripleNybble), //2nnn
    SkipIfVxEqKK(TripleNybble), //3xkk
    SkipIfVxNEqKK(TripleNybble), //4xkk
    SetVxKK(TripleNybble), //6xkk
    VxEqVxPlusKK(TripleNybble), //7xkk
    SetIToNNN(TripleNybble), //Annn
    PCEqNNNPlusV0(TripleNybble), //Bnnn
    VxEqRandANDKK(TripleNybble), //Cxkk
    DrawVxVyNib(TripleNybble), //Dxyn
}


#[derive(Debug)]
struct Nybble([u8; 1]);

impl Nybble {
    fn new(argument: [u8; 1]) -> Self {
        if ((argument[0] & (0b11110000) != 0)) {
            panic!(
                "Invalid nybble value: {:X}. Did your nybble get parsed in correctly?",
                argument[0]
            );
        } else {
            Nybble(argument)
        }
    }
}

impl From<u16> for Nybble {
    fn from(op: u16) -> Nybble {
        Nybble::new(([((op >> 8) & 0x0F) as u8]))
    }
}

#[derive(Debug)]
struct DoubleNybble([u8; 1]);

impl DoubleNybble {
    fn x(&self) -> u8 {
        self.0[0] >> 4
    }

    fn y(&self) -> u8 {
        self.0[0] & 0x0F
    }
}

impl From<u16> for DoubleNybble {
    fn from(op: u16) -> DoubleNybble {
        DoubleNybble([((op >> 4) & 0x0FF) as u8])
    }
}

#[derive(Debug)]
struct TripleNybble([u8; 2]);

impl TripleNybble {
    fn new(argument: [u8; 2]) -> Self {
        if ((argument[0] & (0b11110000) != 0)) {
            panic!(
                "Invalid nybble value: {:X}. Did your three arguments get parsed in correctly?",
                argument[0]
            );
        } else {
            TripleNybble(argument)
        }
    }
    fn to_addr(&self) -> u16 {
        ((((self.0[0] as u16) << 8) | (self.0[1]) as u16))
    }
}

impl From<u16> for TripleNybble {
    fn from(op: u16) -> TripleNybble {
        TripleNybble::new(([((op & 0x0F00) >> 8) as u8, (op & 0x00FF) as u8]))
    }
}

struct Registers {
    pc: ProgramCounter,
    delay: u8,
    sound: u8,
    flag: bool,
    sp: u8,
    i_reg: u16,
    v_regs: [u8; 15],
}

impl Registers {
    fn new() -> Registers {
        let chip_8_adrr = 0x200;
        Registers {
            pc: ProgramCounter(chip_8_adrr),
            delay: 0,
            sound: 0,
            flag: false,
            i_reg: 0,
            sp: 0,
            v_regs: [0; 15],
        }
    }
}

struct Ram([u8; 0xFFF]);

impl Ram {
    fn new() -> Ram {
        Ram { 0: [0; 0xFFF] }
    }
}

struct ProgramCounter(u16);

impl ProgramCounter {
    fn update(&mut self) {
        self.0 += 2;
    }
}

struct Stack([u16; 16]);

impl Stack {
    fn new() -> Stack {
        Stack { 0: [0; 16] }
    }
    fn push(&mut self, sp: &mut u8, pc: &ProgramCounter) {
        *sp = *sp + 1;
        self.0[*sp as usize] = pc.0;
    }

    //  Optional: Delete the data that was popped. By default, right now only the pointer to it gets
    //  deleted. The next time the SP tries to point at the popped stack data, it will simply be
    //  overwritten

    fn pop(&self, sp: &mut u8) -> ProgramCounter {
        let temp = ProgramCounter(self.0[*sp as usize]);
        *sp = *sp - 1;
        temp
    }
}

struct Screen([[bool; 64]; 32]);

impl Screen {
    fn new() -> Screen {
        Screen { 0: [[false; 64]; 32] }
    }
}

//
//   Keyboard will be hashmap, left is key input, right will be true or false. true for pressed
//   false for not pressed
//

fn main() {
    let mut ram: Ram = Ram::new();
    let mut regs: Registers = Registers::new();
    let mut stack: Stack = Stack::new();
    load_rom("dev");
    loop {
        decode_op(fetch_opcode(&regs.pc, &ram));
        regs.pc.update();
    }
}

fn load_rom(file: &str) -> Vec<u8> {
    let mut rom = File::open(file).expect("Rom not found");
    let mut raw_bytes = Vec::new();
    rom.read_to_end(&mut raw_bytes).expect(
        "Something went wrong while reading the rom",
    );
    raw_bytes
}

fn load_rom_into_mem(bytes: Vec<u8>, ram: &mut Ram) {
    &ram.0[0x200..0xFFF].copy_from_slice(&bytes);
}

fn fetch_opcode(pc: &ProgramCounter, ram: &Ram) -> u16 {
    let left_byte: u8 = ram.0[pc.0 as usize];
    let right_byte: u8 = ram.0[(pc.0 + 1) as usize];
    (((left_byte as u16) << 8) | (right_byte as u16))
}

// TODO: Run tests on games and see which opcodes appear most frequently. Reorder this table so
// that the frequent ones are higher in order

fn decode_op(op: u16) -> Opcode {
    match op {
        CLEAR_SCREEN => Opcode::ZeroArg(ZeroArg::ClearScreen),
        RET_SUBROUTINE => Opcode::ZeroArg(ZeroArg::ReturnSubrt),
        op if ((op & READ_V0_VX) == READ_V0_VX) => Opcode::OneArg(
            OneArg::ReadV0Vx(Nybble::from(op)),
        ),
        op if ((op & STORE_V0_VX) == STORE_V0_VX) => Opcode::OneArg(
            OneArg::StoreV0Vx(Nybble::from(op)),
        ),
        op if ((op & STORE_DEC_VX) == STORE_DEC_VX) => Opcode::OneArg(
            OneArg::StoreDecVx(Nybble::from(op)),
        ),
        op if ((op & SET_SPRITE_I) == SET_SPRITE_I) => Opcode::OneArg(
            OneArg::SetSpriteI(Nybble::from(op)),
        ),
        op if ((op & SET_I) == SET_I) => Opcode::OneArg(OneArg::SetI(Nybble::from(op))),
        op if ((op & SET_ST) == SET_ST) => Opcode::OneArg(OneArg::SetST(Nybble::from(op))),
        op if ((op & SET_DT) == SET_DT) => Opcode::OneArg(OneArg::SetDT(Nybble::from(op))),
        op if ((op & WAIT_FOR_KEY) == WAIT_FOR_KEY) => Opcode::OneArg(
            OneArg::WaitForKey(Nybble::from(op)),
        ),
        op if ((op & SET_VX_DT) == SET_VX_DT) => Opcode::OneArg(OneArg::SetVxDT(Nybble::from(op))),
        op if ((op & VX_PLUS_EQ_KK) == VX_PLUS_EQ_KK) => Opcode::ThreeArg(ThreeArg::VxEqVxPlusKK(
            TripleNybble::from(op),
        )),
        op if ((op & VX_EQ_KK) == VX_EQ_KK) => Opcode::ThreeArg(
            ThreeArg::SetVxKK(TripleNybble::from(op)),
        ),
        op if ((op & SKIP_VX_EQ_VY) == SKIP_VX_EQ_VY) => Opcode::TwoArg(
            TwoArg::SkipEqVxVy(DoubleNybble::from(op)),
        ),
        op if ((op & SKIP_VX_NEQ_KK) == SKIP_VX_NEQ_KK) => Opcode::ThreeArg(
            ThreeArg::SkipIfVxNEqKK(
                TripleNybble::from(op),
            ),
        ), 
        op if ((op & SKIP_VX_EQ_KK) == SKIP_VX_EQ_KK) => Opcode::ThreeArg(ThreeArg::SkipIfVxEqKK(
            TripleNybble::from(op),
        )), 
        op if ((op & CALL_SUB_AT_ADDR) == CALL_SUB_AT_ADDR) => Opcode::ThreeArg(
            ThreeArg::CallSubAtNNN(
                TripleNybble::from(op),
            ),
        ), 
        op if ((op & JUMP_TO_ADDR) == JUMP_TO_ADDR) => Opcode::ThreeArg(ThreeArg::JumpToAddrNNN(
            TripleNybble::from(op),
        )), 
        op if ((op & JUMP_TO_CODEROUTE) == JUMP_TO_CODEROUTE) => Opcode::ThreeArg(
            ThreeArg::JumpToCodeRoutNNN(
                TripleNybble::from(op),
            ),
        ),
        op if ((op & SKIP_IF_NOT_VX) == SKIP_IF_NOT_VX) => Opcode::OneArg(
            OneArg::SkipIfNotVx(Nybble::from(op)),
        ),
        op if ((op & SKIP_IF_VX) == SKIP_IF_VX) => Opcode::OneArg(
            OneArg::SkipIfVx(Nybble::from(op)),
        ),
        op if ((op & DRAW_VX_VY_NIB) == DRAW_VX_VY_NIB) => Opcode::ThreeArg(ThreeArg::DrawVxVyNib(
            TripleNybble::from(op),
        )),
        op if ((op & VX_EQ_RAND_PLUS_KK) == VX_EQ_RAND_PLUS_KK) => Opcode::ThreeArg(
            ThreeArg::VxEqRandANDKK(
                TripleNybble::from(op),
            ),
        ),
        op if ((op & PC_EQ_V0_PLUS_NNN) == PC_EQ_V0_PLUS_NNN) => Opcode::ThreeArg(
            ThreeArg::PCEqNNNPlusV0(
                TripleNybble::from(op),
            ),
        ),
        op if ((op & I_EQ_NNN) == I_EQ_NNN) => Opcode::ThreeArg(
            ThreeArg::SetIToNNN(TripleNybble::from(op)),
        ),
        op if ((op & SKIP_IF_VX_NOT_VY) == SKIP_IF_VX_NOT_VY) => Opcode::TwoArg(
            TwoArg::SkipIfVxNotEqVy(
                DoubleNybble::from(op),
            ),
        ),
        op if ((op & SHIFT_VX_L) == SHIFT_VX_L) => Opcode::TwoArg(
            TwoArg::ShiftVxLeft(DoubleNybble::from(op)),
        ),
        op if ((op & VX_EQ_VY_SUB_VX_F) == VX_EQ_VY_SUB_VX_F) => Opcode::TwoArg(
            TwoArg::VxEqVySubVxSetF(
                DoubleNybble::from(op),
            ),
        ),
        op if ((op & SHIFT_VX_R) == SHIFT_VX_R) => Opcode::TwoArg(
            TwoArg::ShiftVxRight(DoubleNybble::from(op)),
        ),
        op if ((op & VX_SUB_EQ_VY_F) == VX_SUB_EQ_VY_F) => Opcode::TwoArg(TwoArg::VxEqVxSubVySetF(
            DoubleNybble::from(op),
        )),
        op if ((op & VX_PLUS_EQ_VY_F) == VX_PLUS_EQ_VY_F) => Opcode::TwoArg(
            TwoArg::VxEqVxPlusVySetF(
                DoubleNybble::from(op),
            ),
        ),
        op if ((op & VX_XOR_EQ_VY) == VX_XOR_EQ_VY) => Opcode::TwoArg(
            TwoArg::VxEqVxXORVy(DoubleNybble::from(op)),
        ),
        op if ((op & VX_AND_EQ_VY) == VX_AND_EQ_VY) => Opcode::TwoArg(
            TwoArg::VxEqVxANDVy(DoubleNybble::from(op)),
        ),
        op if ((op & VX_OR_EQ_VY) == VX_OR_EQ_VY) => Opcode::TwoArg(
            TwoArg::VxEqVxORVy(DoubleNybble::from(op)),
        ),
        op if ((op & VX_EQ_VY) == VX_EQ_VY) => Opcode::TwoArg(
            TwoArg::VxEqVy(DoubleNybble::from(op)),
        ),
        _ => panic!("Unsupported op {:X}", op), 
    }
}

fn execute(
    opcode: Opcode,
    ram: &mut Ram,
    regs: &mut Registers,
    stack: &mut Stack,
    screen: &mut Screen,
) {
    match opcode {
        Opcode::ZeroArg(ZeroArg::ClearScreen) => {
            screen.0.iter_mut().for_each(|inner_array| {
                inner_array.iter_mut().for_each(|pixel| *pixel = false)
            })
        } //00E0 
        Opcode::ZeroArg(ZeroArg::ReturnSubrt) => regs.pc = stack.pop(&mut regs.sp), //00EE
        Opcode::OneArg(OneArg::SkipIfVx(arg)) => skip_if_vx(arg), // Ex9E
        Opcode::OneArg(OneArg::SkipIfNotVx(arg)) => skip_if_not_vx(arg), // ExA1
        Opcode::OneArg(OneArg::SetVxDT(arg)) => regs.v_regs[arg.0[0] as usize] = regs.delay, // Fx07
        Opcode::OneArg(OneArg::WaitForKey(arg)) => load_key_vx(arg), // Fx0A
        Opcode::OneArg(OneArg::SetDT(arg)) => regs.delay = regs.v_regs[arg.0[0] as usize], // Fx15
        Opcode::OneArg(OneArg::SetST(arg)) => regs.sound = regs.v_regs[arg.0[0] as usize], // Fx18
        Opcode::OneArg(OneArg::SetI(arg)) => regs.i_reg += (regs.v_regs[arg.0[0] as usize]) as u16, // Fx1E
        Opcode::OneArg(OneArg::SetSpriteI(arg)) => i_eq_spr_digit_vx(arg), // Fx29
        Opcode::OneArg(OneArg::StoreDecVx(arg)) => {
            store_dec_vx_in_i(ram, regs.i_reg, regs.v_regs[arg.0[0] as usize])
        } // Fx33
        Opcode::OneArg(OneArg::StoreV0Vx(arg)) => {
            store_vx_v0_in_i(arg, ram, &mut regs.v_regs, &regs.i_reg)
        } // Fx55
        Opcode::OneArg(OneArg::ReadV0Vx(arg)) => {
            read_i_in_vx_v0(arg, ram, &mut regs.v_regs, &regs.i_reg)
        } // Fx65
        Opcode::TwoArg(TwoArg::SkipEqVxVy(arg)) => skip_vx_eq_vy(arg, &regs.v_regs, &mut regs.pc), // 5xy0
        Opcode::TwoArg(TwoArg::VxEqVy(arg)) => {
            regs.v_regs[arg.x() as usize] = regs.v_regs[arg.y() as usize]
        } //8xy0
        Opcode::TwoArg(TwoArg::VxEqVxORVy(arg)) => {
            regs.v_regs[arg.x() as usize] |= regs.v_regs[arg.y() as usize]
        }
        Opcode::TwoArg(TwoArg::VxEqVxANDVy(arg)) => {
            regs.v_regs[arg.x() as usize] &= regs.v_regs[arg.y() as usize]
        }
        Opcode::TwoArg(TwoArg::VxEqVxXORVy(arg)) => {
            regs.v_regs[arg.x() as usize] ^= regs.v_regs[arg.y() as usize]
        }
        Opcode::TwoArg(TwoArg::VxEqVxPlusVySetF(arg)) => {
            add_vx_vy_f_carry(
                regs.v_regs[arg.y() as usize],
                &mut regs.v_regs[arg.x() as usize],
                &mut regs.flag,
            )
        } //8xy4
        Opcode::TwoArg(TwoArg::VxEqVxSubVySetF(arg)) => {
            sub_vx_vy_f_nbor(
                regs.v_regs[arg.y() as usize],
                &mut regs.v_regs[arg.x() as usize],
                &mut regs.flag,
            )
        } //8xy5
        Opcode::TwoArg(TwoArg::ShiftVxRight(arg)) => {
            shift_r_vx_vy(&mut regs.flag, &mut regs.v_regs[arg.x() as usize])
        } //8xy6
        Opcode::TwoArg(TwoArg::VxEqVySubVxSetF(arg)) => {
            sub_vy_vx_f_nbor(
                regs.v_regs[arg.y() as usize],
                &mut regs.v_regs[arg.x() as usize],
                &mut regs.flag,
            )
        } //8xy7
        Opcode::TwoArg(TwoArg::ShiftVxLeft(arg)) => {
            shift_l_vx_vy(&mut regs.flag, &mut regs.v_regs[arg.x() as usize])
        } //8xyE
        Opcode::TwoArg(TwoArg::SkipIfVxNotEqVy(arg)) => {
            skip_vx_neq_vy(arg, &mut regs.pc, &mut regs.v_regs)
        } //9xy0
        Opcode::ThreeArg(ThreeArg::JumpToCodeRoutNNN(arg)) => (), //0nnn
        Opcode::ThreeArg(ThreeArg::JumpToAddrNNN(arg)) => regs.pc.0 = arg.to_addr(),
        Opcode::ThreeArg(ThreeArg::CallSubAtNNN(arg)) => stack.push(&mut regs.sp, &regs.pc), //2nnn
        Opcode::ThreeArg(ThreeArg::SkipIfVxEqKK(arg)) => {
            skip_vx_eq_kk(arg, &regs.v_regs, &mut regs.pc)
        } //3xkk
        Opcode::ThreeArg(ThreeArg::SkipIfVxNEqKK(arg)) => {
            skip_vx_neq_kk(arg, &regs.v_regs, &mut regs.pc)
        } //4xkk
        Opcode::ThreeArg(ThreeArg::SetVxKK(arg)) => regs.v_regs[arg.0[0] as usize] = arg.0[1], //6xkk
        Opcode::ThreeArg(ThreeArg::VxEqVxPlusKK(arg)) => regs.v_regs[arg.0[0] as usize] += arg.0[1], //7xkk
        Opcode::ThreeArg(ThreeArg::SetIToNNN(arg)) => regs.i_reg = arg.to_addr(), //Annn
        Opcode::ThreeArg(ThreeArg::PCEqNNNPlusV0(arg)) => {
            regs.pc.0 = (regs.v_regs[0] as u16) + arg.to_addr()
        } //Bnnn
        Opcode::ThreeArg(ThreeArg::VxEqRandANDKK(arg)) => {
            regs.v_regs[arg.0[0] as usize] = arg.0[1] & rand::random::<u8>()
        } //Cxkk
        Opcode::ThreeArg(ThreeArg::DrawVxVyNib(arg)) => draw_vx_vy_nybble(arg), //Dxyn
        _ => panic!("Corrupt or unsupported op"),
    }
}

fn skip_vx_eq_kk(byte_args: TripleNybble, v_regs: &[u8; 15], pc: &mut ProgramCounter) {
    if (v_regs[byte_args.0[0] as usize] == byte_args.0[1]) {
        pc.update();
    }
}

fn skip_vx_neq_kk(byte_args: TripleNybble, v_regs: &[u8; 15], pc: &mut ProgramCounter) {
    if (v_regs[byte_args.0[0] as usize] != byte_args.0[1]) {
        pc.update();
    }
}

fn skip_vx_eq_vy(byte_args: DoubleNybble, v_regs: &[u8; 15], pc: &mut ProgramCounter) {
    if (v_regs[byte_args.x() as usize] == v_regs[byte_args.y() as usize]) {
        pc.update();
    }
}

fn add_vx_vy_f_carry(y_reg: u8, x_reg: &mut u8, flag: &mut bool) {
    *flag = (((y_reg as u16) + (*x_reg as u16)) & 0xFF00) != 0;
    *x_reg = (((*x_reg as u16) + (y_reg as u16)) & 0x00FF) as u8;
}

fn sub_vx_vy_f_nbor(y_reg: u8, x_reg: &mut u8, flag: &mut bool) {
    *flag = *x_reg > y_reg;
    *x_reg = (*x_reg - y_reg);
}

fn shift_r_vx_vy(flag: &mut bool, reg_x: &mut u8) {
    *flag = (0b00000001 & *reg_x == 0b00000001);
    *reg_x /= 2;
}

fn sub_vy_vx_f_nbor(y_reg: u8, x_reg: &mut u8, flag: &mut bool) {
    *flag = *x_reg < y_reg;
    *x_reg = (y_reg - *x_reg);
}

//  Possible optimization of 8xy6 and 8xyE, extract division and multiplication into higher order
//  func.

//  Note: Instructioins 8xyE and 8xy6 change depending on the interpreter. Double check for odd
//  emulator behaviour

fn shift_l_vx_vy(flag: &mut bool, reg_x: &mut u8) {
    *flag = (0b1000000 & *reg_x == 0b10000000);
    *reg_x *= 2;
}

fn skip_vx_neq_vy(byte_args: DoubleNybble, pc: &mut ProgramCounter, v_reg: &mut [u8; 15]) {
    if (v_reg[byte_args.x() as usize] != v_reg[byte_args.y() as usize]) {
        pc.update();
    }
}

fn draw_vx_vy_nybble(byte_args: TripleNybble) {
    println!("Got to opcode {:?}", byte_args);
}

fn skip_if_vx(byte_arg: Nybble) {
    println!("Got to opcode {:?}", byte_arg);
}

fn skip_if_not_vx(byte_arg: Nybble) {
    println!("Got to opcode {:?}", byte_arg);
}

fn load_key_vx(byte_arg: Nybble) {
    println!("Got to opcode {:?}", byte_arg);
}

fn i_eq_spr_digit_vx(byte_arg: Nybble) {
    println!("Got to opcode {:?}", byte_arg);
}

fn store_dec_vx_in_i(ram: &mut Ram, i_reg: u16, v_reg: u8) {
    ram.0[i_reg as usize] = v_reg / 100;
    ram.0[(i_reg + 1) as usize] = (v_reg % 100) / 10;
    ram.0[(i_reg + 2) as usize] = v_reg % 10;
}

fn store_vx_v0_in_i(byte_arg: Nybble, ram: &mut Ram, v_regs: &mut [u8; 15], i_reg: &u16) {
    for index in 0..byte_arg.0[0] {
        v_regs[index as usize] = ram.0[(*i_reg + index as u16) as usize];
    }
}

fn read_i_in_vx_v0(byte_arg: Nybble, ram: &mut Ram, v_regs: &mut [u8; 15], i_reg: &u16) {
    for index in 0..byte_arg.0[0] {
        ram.0[(*i_reg + index as u16) as usize] = v_regs[index as usize]
    }
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
    assert_eq!(0x0FBA, TripleNybble::new([0x0F, 0xBA]).to_addr())
}

#[test]
#[should_panic]
fn test_nybble() {
    let nybble: Nybble = Nybble::new([0xFA]);
}

#[test]
#[should_panic]
fn test_triple_nybble() {
    let tnybble: TripleNybble = TripleNybble::new([0xFA, 0xFD]);
}

#[test]
fn test_decode_op() {
    let chip8_addr: u16 = 0x200;
    let amount_of_ops: u16 = 35;
    let mut ram: Ram = Ram::new();
    let mut regs: Registers = Registers::new();
    let test_ops: [u8; 70] = [
        0x05,
        0x55,
        0x00,
        0xE0,
        0x00,
        0xEE,
        0x15,
        0x55,
        0x25,
        0x55,
        0x31,
        0x33,
        0x41,
        0x33,
        0x56,
        0x70,
        0x61,
        0x33,
        0x71,
        0x33,
        0x86,
        0x70,
        0x86,
        0x71,
        0x86,
        0x72,
        0x86,
        0x73,
        0x86,
        0x74,
        0x86,
        0x75,
        0x86,
        0x76,
        0x86,
        0x77,
        0x86,
        0x7E,
        0x96,
        0x70,
        0xA5,
        0x55,
        0xB5,
        0x55,
        0xC1,
        0x33,
        0xD6,
        0x75,
        0xE1,
        0x9E,
        0xE1,
        0xA1,
        0xF1,
        0x07,
        0xF1,
        0x0A,
        0xF1,
        0x15,
        0xF1,
        0x18,
        0xF1,
        0x1E,
        0xF1,
        0x29,
        0xF1,
        0x33,
        0xF1,
        0x55,
        0xF1,
        0x65,
    ];
    let mut x = chip8_addr;
    for element in test_ops.iter() {
        ram.0[x as usize] = *element;
        x += 1;
    }
    loop {
        decode_op(fetch_opcode(&regs.pc, &ram));
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

#[test]
fn test_clear_screen() {
    let mut ram: Ram = Ram::new();
    let mut regs: Registers = Registers::new();
    let mut stack: Stack = Stack::new();
    let mut screen: Screen = Screen::new();
    screen.0[0][0] = true;
    execute(
        decode_op(CLEAR_SCREEN),
        &mut ram,
        &mut regs,
        &mut stack,
        &mut screen,
    );
    screen.0.iter().for_each(|inner_array| {
        inner_array.iter().for_each(
            |pixel| assert_eq!(*pixel, false),
        )
    });
}
