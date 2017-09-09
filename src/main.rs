struct Interpreter {
    registers: Registers,
}

struct Nybble(u8);

impl Nybble {
    fn new(argument: u8 ) -> Self {
        if ((argument & (0b11110000) != 0)) {
            panic!("Invalid nybble value: {:X}. Did your nybble get parsed in correctly?", argument);
        }
        else {
            Nybble {
                0: argument,
            }
        }
    }
}

struct Registers {
    program_counter: ProgramCounter,
    delay: u8,
    sound: u8,
    flag: u8,
    stackPointer: u8,
    iRegister: u16,
    vRegisters: [u8; 15],
}

impl Registers {
    fn new() -> Registers {
        let chip8Adrr = 0x200;
        Registers {
            program_counter: ProgramCounter(chip8Adrr),
            delay: 0,
            sound: 0,
            flag: 0,
            iRegister: 0,
            stackPointer: 0,
            vRegisters: [0; 15],
        }
    }
}

struct Ram {
    whole_bank: [u8; 0xFFF],
}

impl Ram {
    fn new() -> Ram {
        Ram {
           whole_bank:  [0; 0xFFF],
        }
    }
}

struct ProgramCounter(u8);

impl ProgramCounter {
    fn update_counter(&mut self){
        self.0+=2;
    }
}

fn main() {
    let mut ram: Ram = Ram::new();
    let mut registers: Registers = Registers::new();
    load_rom("dev");
    loop {
        decode_execute_op(
        fetch_opcode(&registers.program_counter, &ram));
        registers.program_counter.update_counter();
    }
}

fn load_rom(path: &str) -> () {
    unimplemented!();
}

fn fetch_opcode(pc: &ProgramCounter, ram: &Ram) -> u16 {
//  TODO: implement slice references for u8
    let left_byte: u8 = ram.whole_bank[pc.0 as usize];
    let right_byte: u8 = ram.whole_bank[(pc.0 + 1) as usize];
    (((left_byte as u16) << 8) | (right_byte as u16))
}

fn decode_execute_op(opcode: u16) -> () {
//  First check to see if opcode has operands
    match opcode {
        0x00E0 => println!("Got to opcode {:X}", opcode),
        0x00EE => println!("Got to opcode {:X}", opcode),
        _ => match (opcode & 0xF000) {
            0xF000 => match (opcode & 0x00F0) {
                0x0060 => println!("Got to opcode {:X}", opcode),
                0x0050 => println!("Got to opcode {:X}", opcode),
                0x0030 => println!("Got to opcode {:X}", opcode),
                0x0020 => println!("Got to opcode {:X}", opcode),
                0x0010 => match (opcode & 0x000F) {
                    0x000E => println!("Got to opcode {:X}", opcode),
                    0x0008 => println!("Got to opcode {:X}", opcode),
                    0x0005 => println!("Got to opcode {:X}", opcode),
                    _ => panic!("Unsupported or corrupt opcode"),
                    },
                0x0000 => match (opcode & 0x000F) {
                    0x000A => println!("Got to opcode {:X}", opcode),
                    0x0007 => println!("Got to opcode {:X}", opcode),
                    0x0001 => println!("Got to opcode {:X}", opcode),
                    _ => panic!("Unsupported or corrupt opcode"),
                    },
                _ => panic!("Unsupported or corrupt opcode"),
            },
            0xE000 => match (opcode & 0x00F0) {
                0x00A0 => println!("Got to opcode {:X}", opcode),
                0x0090 => println!("Got to opcode {:X}", opcode),
                _ => panic!("unsupported or Corrupt opcode"),
            },
            0xD000 => println!("Got to opcode {:X}", opcode),
            0xC000 => println!("Got to opcode {:X}", opcode),
            0xB000 => println!("Got to opcode {:X}", opcode),
            0xA000 => println!("Got to opcode {:X}", opcode),
            0x9000 => println!("Got to opcode {:X}", opcode),
            0x8000 => match (opcode & 0x000E) {
                0x000E => println!("Got to opcode {:X}", opcode),
                0x0007 => println!("Got to opcode {:X}", opcode),
                0x0006 => println!("Got to opcode {:X}", opcode),
                0x0005 => println!("Got to opcode {:X}", opcode),
                0x0004 => println!("Got to opcode {:X}", opcode),
                0x0003 => println!("Got to opcode {:X}", opcode),
                0x0002 => println!("Got to opcode {:X}", opcode),
                0x0001 => println!("Got to opcode {:X}", opcode),
                0x0000 => println!("Got to opcode {:X}", opcode),
                _ => panic!("Unsupported or corrupt opcode"),
            },
            0x7000 => println!("Got to opcode {:X}", opcode),
            0x6000 => println!("Got to opcode {:X}", opcode),
            0x5000 => println!("Got to opcode {:X}", opcode),
            0x4000 => println!("Got to opcode {:X}", opcode),
            0x3000 => println!("Got to opcode {:X}", opcode),
            0x2000 => println!("Got to opcode {:X}", opcode),
            0x1000 => println!("Got to opcode {:X}", opcode),
            0x0000 => println!("Got to opcode {:X}", opcode),
            _ => panic!("Unsupported or corrupt opcode"),
        }
    }
}
//fn extract_operand(opcode: u16) -> Nybble {
//
//}
fn clear_screen() -> () {
    unimplemented!();
}

fn ret_subroutine() -> () {
    unimplemented!();
}

fn load_vx_i() -> () {
    unimplemented!();
}

fn load_i_vx() -> () {
    unimplemented!();
}

fn load_b_vx() -> () {
    unimplemented!();
}

fn load_f_vx() -> () {
    unimplemented!();
}

fn add_i_vx() -> () {
    unimplemented!();
}

fn load_sound_timer_vx() -> () {
    unimplemented!();
}

#[test]
fn fetch_opcode_test() {
    let mut ram: Ram = Ram::new();
    let registers: Registers = Registers::new();
    ram.whole_bank[0] = 0xFF;
    ram.whole_bank[1] = 0xA2;
    assert_eq!(fetch_opcode(&registers.program_counter, &ram),0xFFA2);

}

#[test]
#[should_panic]
fn test_nybble() {
    let nybble: Nybble = Nybble::new(0xFA);
}

#[test]
fn test_decode_execute_op() {
    let mut ram: Ram = Ram::new();
    let mut registers: Registers = Registers::new();
    let test_ops: [u8; 70] = [0x05, 0x55, 0x00, 0xE0, 0x00, 0xEE, 0x15, 0x55, 0x25, 0x55, 0x31, 0x33, 0x41, 0x33, 0x56, 0x70, 0x61, 0x33, 0x71, 0x33, 0x86, 0x70, 0x86, 0x71, 0x86, 0x72, 0x86, 0x73, 0x86, 0x74, 0x86, 0x75, 0x86, 0x76, 0x86, 0x77, 0x86, 0x7E, 0x96, 0x70, 0xA5, 0x55, 0xB5, 0x55, 0xC1, 0x33, 0xD6, 0x75, 0xE1, 0x9E, 0xE1, 0xA1, 0xF1, 0x07, 0xF1, 0x0A, 0xF1, 0x15, 0xF1, 0x18, 0xF1, 0x1E, 0xF1, 0x29, 0xF1, 0x33, 0xF1, 0x55, 0xF1, 0x65];
    let mut x = 0;
    for element in test_ops.iter() {
        ram.whole_bank[x] = *element;
        x+=1;
    }
    loop {
        decode_execute_op(fetch_opcode(&registers.program_counter, &ram));
        registers.program_counter.update_counter();
        if (registers.program_counter.0 == 70) {
            break;
        }
    }
}



//Testing -Jason
