extern crate rand;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct Interpreter {
    registers: Registers,
}

enum Opcode {
    ZeroArg(ZeroArg),
    OneArg(OneArg),
    TwoArg(TwoArg),
    ThreeArg(ThreeArg),
}

enum ZeroArg {
    ClearScreen, //00E0
    ReturnSubrt, //00EE
}

enum OneArg {
    SkipIfVx(Nybble), // Ex9E
    SkipIfNotVx(Nybble), // ExA1
    SetVxDT(Nybble), // Fx07
    WaitForKey(Nybble), // Fx0A
    SetDT(Nybble), // Fx15
    SetST(Nybble), // Fx18
    SetI(Nybble), // Fx1E
    SetSpriteI(Nybble), // Fx29
    StoreDecVx(Nybble), // Fx33
    StoreV0Vx(Nybble), // Fx55
    ReadV0Vx(Nybble), // Fx65
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
}

struct Registers {
    program_counter: ProgramCounter,
    delay: u8,
    sound: u8,
    flag: bool,
    sp: u8,
    i_register: u16,
    v_registers: [u8; 15],
}

impl Registers {
    fn new() -> Registers {
        let chip_8_adrr = 0x200;
        Registers {
            program_counter: ProgramCounter(chip_8_adrr),
            delay: 0,
            sound: 0,
            flag: false,
            i_register: 0,
            sp: 0,
            v_registers: [0; 15],
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
 
struct Stack([u16; 16]);

impl Stack {
    fn new() -> Stack {
        Stack { 0: [0; 16]}
    }
}
impl ProgramCounter {
    fn update_counter(&mut self) {
        self.0 += 2;
    }
}



//
//   Keyboard will be hashmap, left is key input, right will be true or false. true for pressed
//   false for not pressed
//



fn main() {
    let mut ram: Ram = Ram::new();
    let mut registers: Registers = Registers::new();
    let mut stack: Stack = Stack::new();
    load_rom("dev");
    loop {
        decode_op(fetch_opcode(&registers.program_counter, &ram));
        registers.program_counter.update_counter();
    }
}

fn load_rom(file: &str) -> Vec<u8> {
    let mut rom = File::open(file).expect("Rom not found");
    let mut raw_bytes = Vec::new();
    rom.read_to_end(&mut raw_bytes).expect("Something went wrong while reading the rom");
    raw_bytes
}

fn load_rom_into_mem(bytes: Vec<u8>, ram: &mut Ram) {
   &ram.0[0x200 .. 0xFFF].copy_from_slice(&bytes); 
}

fn fetch_opcode(pc: &ProgramCounter, ram: &Ram) -> u16 {
    let left_byte: u8 = ram.0[pc.0 as usize];
    let right_byte: u8 = ram.0[(pc.0 + 1) as usize];
    (((left_byte as u16) << 8) | (right_byte as u16))
}

fn extract_triple(opcode: u16) -> TripleNybble {
    TripleNybble::new(([((opcode & 0x0F00) >> 8) as u8, (opcode & 0x00FF) as u8]))
}

fn extract_double(opcode: u16) -> DoubleNybble {
    DoubleNybble([((opcode >> 4) & 0x0FF) as u8])
}

fn extract_single(opcode: u16) -> Nybble {
    Nybble::new(([((opcode >> 8) & 0x0F) as u8]))
}

fn triple_nyb_to_addr(arg: &TripleNybble) -> u16 {
    ((((arg.0[0] as u16) << 8) | (arg.0[1]) as u16))  
}


fn decode_op(opcode: u16) -> Opcode {
    match opcode {
        0x00E0 => Opcode::ZeroArg(ZeroArg::ClearScreen),
        0x00EE => Opcode::ZeroArg(ZeroArg::ReturnSubrt),
        _ => {
            match (opcode & 0xF000) {
                0xF000 => {
                    match (opcode & 0x00F0) {
                        0x0060 => Opcode::OneArg(OneArg::ReadV0Vx(extract_single(opcode))),
                        0x0050 => Opcode::OneArg(OneArg::StoreV0Vx(extract_single(opcode))),
                        0x0030 => Opcode::OneArg(OneArg::StoreDecVx(extract_single(opcode))),
                        0x0020 => Opcode::OneArg(OneArg::SetSpriteI(extract_single(opcode))),
                        0x0010 => {
                            match (opcode & 0x000F) {
                                0x000E => Opcode::OneArg(OneArg::SetI(extract_single(opcode))),
                                0x0008 => Opcode::OneArg(OneArg::SetST(extract_single(opcode))),
                                0x0005 => Opcode::OneArg(OneArg::SetDT(extract_single(opcode))),
                                _ => panic!("Unsupported or corrupt opcode"),
                            }
                        }
                        0x0000 => {
                            match (opcode & 0x000F) {
                                0x000A => Opcode::OneArg(OneArg::WaitForKey(extract_single(opcode))),
                                0x0007 => Opcode::OneArg(OneArg::SetVxDT(extract_single(opcode))),
                                _ => panic!("Unsupported or corrupt opcode"),
                            }
                        }
                        _ => panic!("Unsupported or corrupt opcode"),
                    }
                }
                0xE000 => {
                    match (opcode & 0x00F0) {
                        0x00A0 => Opcode::OneArg(OneArg::SkipIfNotVx(extract_single(opcode))),
                        0x0090 => Opcode::OneArg(OneArg::SkipIfVx(extract_single(opcode))),
                        _ => panic!("unsupported or Corrupt opcode"),
                    }
                }
                0xD000 => Opcode::ThreeArg(ThreeArg::DrawVxVyNib(extract_triple(opcode))),
                0xC000 => Opcode::ThreeArg(ThreeArg::VxEqRandANDKK(extract_triple(opcode))),
                0xB000 => Opcode::ThreeArg(ThreeArg::PCEqNNNPlusV0(extract_triple(opcode))),
                0xA000 => Opcode::ThreeArg(ThreeArg::SetIToNNN(extract_triple(opcode))),
                0x9000 => Opcode::TwoArg(TwoArg::SkipIfVxNotEqVy(extract_double(opcode))),
                0x8000 => {
                    match (opcode & 0x000E) {
                        0x000E => Opcode::TwoArg(TwoArg::ShiftVxLeft(extract_double(opcode))),
                        0x0007 => Opcode::TwoArg(TwoArg::VxEqVySubVxSetF(extract_double(opcode))),
                        0x0006 => Opcode::TwoArg(TwoArg::ShiftVxRight(extract_double(opcode))),
                        0x0005 => Opcode::TwoArg(TwoArg::VxEqVxSubVySetF(extract_double(opcode))),
                        0x0004 => Opcode::TwoArg(TwoArg::VxEqVxPlusVySetF(extract_double(opcode))),
                        0x0003 => Opcode::TwoArg(TwoArg::VxEqVxXORVy(extract_double(opcode))),
                        0x0002 => Opcode::TwoArg(TwoArg::VxEqVxANDVy(extract_double(opcode))),
                        0x0001 => Opcode::TwoArg(TwoArg::VxEqVxORVy(extract_double(opcode))),
                        0x0000 => Opcode::TwoArg(TwoArg::VxEqVy(extract_double(opcode))),
                        _ => panic!("Unsupported or corrupt opcode"),
                    }
                }
                0x7000 => Opcode::ThreeArg(ThreeArg::VxEqVxPlusKK(extract_triple(opcode))),
                0x6000 => Opcode::ThreeArg(ThreeArg::SetVxKK(extract_triple(opcode))),
                0x5000 => Opcode::TwoArg(TwoArg::SkipEqVxVy(extract_double(opcode))),
                0x4000 => Opcode::ThreeArg(ThreeArg::SkipIfVxNEqKK(extract_triple(opcode))), 
                0x3000 => Opcode::ThreeArg(ThreeArg::SkipIfVxEqKK(extract_triple(opcode))), 
                0x2000 => Opcode::ThreeArg(ThreeArg::CallSubAtNNN(extract_triple(opcode))), 
                0x1000 => Opcode::ThreeArg(ThreeArg::JumpToAddrNNN(extract_triple(opcode))), 
                0x0000 => Opcode::ThreeArg(ThreeArg::JumpToCodeRoutNNN(extract_triple(opcode))),
                _ => panic!("Unsupported or corrupt opcode"),
            }
        }
    }
}

fn execute(opcode: Opcode, ram: &mut Ram, registers: &mut Registers, stack: &mut Stack) {
    match opcode {
        Opcode::ZeroArg(ZeroArg::ClearScreen) => clear_screen(), //00E0
        Opcode::ZeroArg(ZeroArg::ReturnSubrt) => ret_subroutine(&mut registers.program_counter, &stack, &mut registers.sp),  //00EE
        Opcode::OneArg(OneArg::SkipIfVx(arg)) =>  skip_if_vx(arg), // Ex9E
        Opcode::OneArg(OneArg::SkipIfNotVx(arg)) =>  skip_if_not_vx(arg), // ExA1
        Opcode::OneArg(OneArg::SetVxDT(arg)) =>  registers.v_registers[arg.0[0] as usize] = registers.delay, // Fx07
        Opcode::OneArg(OneArg::WaitForKey(arg)) =>  load_key_vx(arg), // Fx0A
        Opcode::OneArg(OneArg::SetDT(arg)) => registers.delay = registers.v_registers[arg.0[0] as usize], // Fx15
        Opcode::OneArg(OneArg::SetST(arg)) => registers.sound = registers.v_registers[arg.0[0] as usize], // Fx18
        Opcode::OneArg(OneArg::SetI(arg)) =>  registers.i_register += (registers.v_registers[arg.0[0] as usize]) as u16, // Fx1E
        Opcode::OneArg(OneArg::SetSpriteI(arg)) => i_eq_spr_digit_vx(arg), // Fx29
        Opcode::OneArg(OneArg::StoreDecVx(arg)) => store_dec_vx_in_i(ram, registers.i_register, registers.v_registers[arg.0[0] as usize]), // Fx33
        Opcode::OneArg(OneArg::StoreV0Vx(arg)) => store_vx_v0_in_i(arg, ram, &mut registers.v_registers, &registers.i_register), // Fx55
        Opcode::OneArg(OneArg::ReadV0Vx(arg)) =>  read_i_in_vx_v0(arg, ram, &mut registers.v_registers, &registers.i_register), // Fx65
        Opcode::TwoArg(TwoArg::SkipEqVxVy(arg)) => skip_vx_eq_vy(arg, &registers.v_registers, &mut registers.program_counter), // 5xy0
        Opcode::TwoArg(TwoArg::VxEqVy(arg)) => registers.v_registers[arg.x() as usize] = registers.v_registers[arg.y() as usize], //8xy0
        Opcode::TwoArg(TwoArg::VxEqVxORVy(arg)) => registers.v_registers[arg.x() as usize] |= registers.v_registers[arg.y() as usize],
        Opcode::TwoArg(TwoArg::VxEqVxANDVy(arg)) => registers.v_registers[arg.x() as usize] &= registers.v_registers[arg.y() as usize],
        Opcode::TwoArg(TwoArg::VxEqVxXORVy(arg)) => registers.v_registers[arg.x() as usize] ^= registers.v_registers[arg.y() as usize],
        Opcode::TwoArg(TwoArg::VxEqVxPlusVySetF(arg)) => add_vx_vy_f_carry(arg, &mut registers.v_registers, &mut registers.flag) , //8xy4
        Opcode::TwoArg(TwoArg::VxEqVxSubVySetF(arg)) => sub_vx_vy_f_nbor(arg, &mut registers.v_registers, &mut registers.flag) , //8xy5
        Opcode::TwoArg(TwoArg::ShiftVxRight(arg)) => shift_r_vx_vy(&mut registers.flag, &mut registers.v_registers[arg.x() as usize]), //8xy6
        Opcode::TwoArg(TwoArg::VxEqVySubVxSetF(arg)) => sub_vy_vx_f_nbor(arg, &mut registers.v_registers, &mut registers.flag) , //8xy7
        Opcode::TwoArg(TwoArg::ShiftVxLeft(arg)) => shift_l_vx_vy(&mut registers.flag, &mut registers.v_registers[arg.x() as usize]) , //8xyE
        Opcode::TwoArg(TwoArg::SkipIfVxNotEqVy(arg)) => skip_vx_neq_vy(arg, &mut registers.program_counter, &mut registers.v_registers) , //9xy0
        Opcode::ThreeArg(ThreeArg::JumpToCodeRoutNNN(arg)) => (), //0nnn
        Opcode::ThreeArg(ThreeArg::JumpToAddrNNN(arg)) => registers.program_counter.0 = triple_nyb_to_addr(&arg),
        Opcode::ThreeArg(ThreeArg::CallSubAtNNN(arg)) => call_addr_nnn(arg, &mut registers.program_counter, stack, &mut registers.sp) , //2nnn
        Opcode::ThreeArg(ThreeArg::SkipIfVxEqKK(arg)) => skip_vx_eq_kk(arg, &registers.v_registers, &mut registers.program_counter) , //3xkk
        Opcode::ThreeArg(ThreeArg::SkipIfVxNEqKK(arg)) => skip_vx_neq_kk(arg, &registers.v_registers, &mut registers.program_counter) , //4xkk
        Opcode::ThreeArg(ThreeArg::SetVxKK(arg)) => registers.v_registers[arg.0[0] as usize] = arg.0[1], //6xkk
        Opcode::ThreeArg(ThreeArg::VxEqVxPlusKK(arg)) => registers.v_registers[arg.0[0] as usize] += arg.0[1], //7xkk
        Opcode::ThreeArg(ThreeArg::SetIToNNN(arg)) => registers.i_register = triple_nyb_to_addr(&arg), //Annn
        Opcode::ThreeArg(ThreeArg::PCEqNNNPlusV0(arg)) => registers.program_counter.0 = (registers.v_registers[0] as u16) + triple_nyb_to_addr(&arg), //Bnnn
        Opcode::ThreeArg(ThreeArg::VxEqRandANDKK(arg)) => registers.v_registers[arg.0[0] as usize] = arg.0[1] & rand::random::<u8>(), //Cxkk
        Opcode::ThreeArg(ThreeArg::DrawVxVyNib(arg)) => draw_vx_vy_nybble(arg) , //Dxyn
        _ => panic!("Corrupt or unsupported op"),
    }
}

fn clear_screen() {  //00E0
    println!("Got to opcode {}" , "00E0");
}

fn ret_subroutine(pc: &mut ProgramCounter, stack: &Stack, sp: &mut u8) {  //00EE
    pc.0 = stack.0[*sp as usize];
    *sp-=1;
}

fn call_addr_nnn(addr: TripleNybble, pc: &mut ProgramCounter, stack: &mut Stack, sp: &mut u8) { //2nnn
    *sp+=1;
    stack.0[*sp as usize] = pc.0;
    pc.0 = triple_nyb_to_addr(&addr);
}

fn skip_vx_eq_kk(byte_args: TripleNybble, v_registers: &[u8; 15], pc: &mut ProgramCounter) {  //3xkk
    if (v_registers[byte_args.0[0] as usize] == byte_args.0[1]) {
        pc.update_counter();
    }
}

fn skip_vx_neq_kk(byte_args: TripleNybble, v_registers: &[u8; 15], pc: &mut ProgramCounter) {  //4xkk
    if (v_registers[byte_args.0[0] as usize] != byte_args.0[1]) {
        pc.update_counter();
    }
}

fn skip_vx_eq_vy(byte_args: DoubleNybble, v_registers: &[u8; 15], pc: &mut ProgramCounter) {  // 5xy0
    if (v_registers[byte_args.x() as usize] == v_registers[byte_args.y() as usize]) {
        pc.update_counter();
    }
}

fn add_vx_vy_f_carry(byte_args: DoubleNybble, y_reg: u8, x_reg: &mut u8, flag: &mut bool) {  //8xy4
    *flag = ((y_reg as u16) + (*x_reg as u16)) & 0xFF00) != 0;
    *x_reg = ((*x_reg as u16) + (y_reg as u16)) & 0x00FF) as u8;
}

fn sub_vx_vy_f_nbor(byte_args: DoubleNybble, y_reg: u8, x_reg: &mut u8, flag: &mut bool) {  //8xy5
    *flag = *x_reg > y_reg;
    *x_reg = (*x_reg - y_reg).abs();
}

fn shift_r_vx_vy(flag: &mut bool, reg_x: &mut u8) { //8xy6
    *flag = (0b00000001 & *reg_x == 0b00000001);
    *reg_x /= 2;
}

fn sub_vy_vx_f_nbor(byte_args: DoubleNybble, y_reg: u8, x_reg: &mut u8, flag: &mut bool) {  //8xy7
    *flag = *x_reg < y_reg;
    *x_reg = (y_reg - *x_reg).abs();
}

//  Possible optimization of 8xy6 and 8xyE, extract division and multiplication into higher order
//  func.

//  Note: Instructioins 8xyE and 8xy6 change depending on the interpreter. Double check for odd
//  emulator behaviour

fn shift_l_vx_vy(flag: &mut bool, reg_x: &mut u8) {  //8xyE
    *flag = (0b1000000 & *reg_x == 0b10000000);
    *reg_x *= 2;
}

fn skip_vx_neq_vy(byte_args: DoubleNybble, pc: &mut ProgramCounter, v_reg: &mut [u8; 15]) {  //9xy0
    if (v_reg[byte_args.x() as usize] != v_reg[byte_args.y() as usize]) {
        pc.update_counter();
    }
}

fn draw_vx_vy_nybble(byte_args: TripleNybble) { //Dxyn
    println!("Got to opcode {:?}" , byte_args);
}

fn skip_if_vx(byte_arg: Nybble) { // Ex9E
    println!("Got to opcode {:?}" , byte_arg);
}

fn skip_if_not_vx(byte_arg: Nybble) {  // ExA1
    println!("Got to opcode {:?}" , byte_arg);
}

fn load_key_vx(byte_arg: Nybble) {  // Fx0A
    println!("Got to opcode {:?}" , byte_arg);
}

fn i_eq_spr_digit_vx(byte_arg: Nybble) {  // Fx29
    println!("Got to opcode {:?}" , byte_arg);
}

fn store_dec_vx_in_i(ram: &mut Ram, i_reg: u16, v_reg: u8) {  // Fx33
    ram.0[i_reg as usize] = v_reg / 100;
    ram.0[(i_reg+1) as usize] = (v_reg % 100) / 10;
    ram.0[(i_reg+2) as usize] = v_reg % 10;
}

fn store_vx_v0_in_i(byte_arg: Nybble, ram: &mut Ram, v_registers: &mut [u8; 15], i_reg: & u16) {  // Fx55
    for index in 0..byte_arg.0[0] {
        v_registers[index as usize] = ram.0[(*i_reg + index as u16) as usize];
    }
}

fn read_i_in_vx_v0(byte_arg: Nybble, ram: &mut Ram, v_registers: &mut [u8; 15], i_reg: & u16) {  // Fx55
    for index in 0..byte_arg.0[0] {
        ram.0[(*i_reg + index as u16) as usize] = v_registers[index as usize]
    }
}


#[test]
fn test_fetch_opcode() {
    let mut ram: Ram = Ram::new();
    let mut registers: Registers = Registers::new();
    registers.program_counter.0 = 0;
    ram.0[0] = 0xFF;
    ram.0[1] = 0xA2;
    assert_eq!(fetch_opcode(&registers.program_counter, &ram), 0xFFA2);

}

#[test]
fn test_jump_addr_nnn() {
    let mut registers: Registers = Registers::new();
    jump_addr_nnn(TripleNybble::new([0x0F,0xAC]), &mut registers.program_counter);
    assert_eq!(registers.program_counter.0, 0x0FAC)
}

#[test]
fn test_triple_nyb_to_addr() {
    assert_eq!(0x0FBA, triple_nyb_to_addr(&TripleNybble::new([0x0F,0xBA])))
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
    let mut registers: Registers = Registers::new();
    let test_ops: [u8; 70] = [
        0x05, 0x55,
        0x00, 0xE0,
        0x00, 0xEE,
        0x15, 0x55,
        0x25, 0x55,
        0x31, 0x33,
        0x41, 0x33,
        0x56, 0x70,
        0x61, 0x33,
        0x71, 0x33,
        0x86, 0x70,
        0x86, 0x71,
        0x86, 0x72,
        0x86, 0x73,
        0x86, 0x74,
        0x86, 0x75,
        0x86, 0x76,
        0x86, 0x77,
        0x86, 0x7E,
        0x96, 0x70,
        0xA5, 0x55,
        0xB5, 0x55,
        0xC1, 0x33,
        0xD6, 0x75,
        0xE1, 0x9E,
        0xE1, 0xA1,
        0xF1, 0x07,
        0xF1, 0x0A,
        0xF1, 0x15,
        0xF1, 0x18,
        0xF1, 0x1E,
        0xF1, 0x29,
        0xF1, 0x33,
        0xF1, 0x55,
        0xF1, 0x65,
    ];
    let mut x = chip8_addr;
    for element in test_ops.iter() {
        ram.0[x as usize] = *element;
        x += 1;
    }
    loop {
        decode_op(fetch_opcode(&registers.program_counter, &ram));
        registers.program_counter.update_counter();
        if (registers.program_counter.0 == (chip8_addr + (amount_of_ops * 2))) {
            break;
        }
    }
}

#[test]
fn test_rom_loader() {
    panic!();
}

#[test]
fn test_single_extracter() {
    assert_eq!(extract_single(0xF1AB).0[0] , 1);
}

#[test]
fn test_double_extracter() {
    assert_eq!(extract_double(0xF1AB).0[0] , 0x1A);
}

#[test]
fn test_triple_extracter() {
    let test: TripleNybble = extract_triple(0xF1AB);
    assert_eq!(test.0[0] , 0x01);
    assert_eq!(test.0[1] , 0xAB);
}

#[test]
#[should_panic]
fn test_wrong_rom_path() {
}

#[test]
fn test_rom_ram_loader() {
    panic!();
}

#[test]
fn test_skip_vx_eq_kk() {
    let mut registers = Registers::new();
    let test = extract_triple(0x5DFA);
    registers.v_registers[0xD] = 0xFA;
    skip_vx_eq_kk(test, &registers.v_registers, &mut registers.program_counter);
    assert_eq!(registers.program_counter.0, 0x202)
}

#[test]
fn test_skip_vx_neq_kk() {
    let mut registers = Registers::new();
    let test = extract_triple(0x5DFA);
    registers.v_registers[0xD] = 0xBA;
    skip_vx_neq_kk(test, &registers.v_registers, &mut registers.program_counter);
    assert_eq!(registers.program_counter.0, 0x202)
}

#[test]
fn test_skip_vx_eq_vy() {
    let mut registers = Registers::new();
    let test = extract_double(0x50D0);
    registers.v_registers[0x0] = 10;
    registers.v_registers[0xD] = 10;
    skip_vx_eq_vy(test, &registers.v_registers, &mut registers.program_counter);
    assert_eq!(registers.program_counter.0, 0x202)
}

#[test]
fn test_load_i_addr() { //Annn
    let mut registers = Registers::new();
    let test = extract_triple(0xABA0);
    load_i_addr(test, &mut registers.i_register);
    assert_eq!(registers.i_register , 0xBA0);
}


#[test]
fn test_load_vx_kk() {
    let mut registers = Registers::new();
    let test = extract_triple(0xB3B0);
    load_vx_kk(test, &mut registers.v_registers);
    assert_eq!(registers.v_registers[3], 0xB0)
}


#[test]
fn test_load_vy_in_vx() {
    let mut registers = Registers::new();
    let test = extract_double(0x8AB0);
    registers.v_registers[0xB] = 0xA;
    load_vy_in_vx(test, &mut registers.v_registers);
    assert_eq!(registers.v_registers[0xA] , 0xA)
}

#[test]
fn test_or_vx_vy() {
    let mut registers = Registers::new();
    let test = extract_double(0x0AB0);
    registers.v_registers[0xA] = 1;
    registers.v_registers[0xB] = 3;
    let y = registers.v_registers[0xB];
    bitop_vx_vy(&mut registers.v_registers[0xA], y, std::ops::BitOr::bitor);
    assert_eq!(registers.v_registers[0xA] , 3)
}

#[test]
fn test_and_vx_vy() {
    let mut registers = Registers::new();
    let test = extract_double(0x0AB0);
    registers.v_registers[0xA] = 1;
    registers.v_registers[0xB] = 3;
    let y = registers.v_registers[0xB];
    bitop_vx_vy(&mut registers.v_registers[0xA], y, std::ops::BitAnd::bitand);
    assert_eq!(registers.v_registers[0xA] , 1)
}

#[test]
fn test_xor_vx_vy() {
    let mut registers = Registers::new();
    let test = extract_double(0x0AB0);
    registers.v_registers[0xA] = 5;
    registers.v_registers[0xB] = 7;
    let y = registers.v_registers[0xB];
    bitop_vx_vy(&mut registers.v_registers[0xA], y, std::ops::BitXor::bitxor);
    assert_eq!(registers.v_registers[0xA] , 2)
}

#[test]
fn test_jump_v0_addr_nnn() {
    let mut registers = Registers::new();
    let test = extract_triple(0xBAD0);
    registers.v_registers[0] = 2;
    jump_v0_addr_nnn(test, &mut registers.program_counter, registers.v_registers[0]);
    assert_eq!(registers.program_counter.0 , 0xAD2)
}