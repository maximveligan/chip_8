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
        0x00E0 => clear_screen(),
        0x00EE => ret_subroutine(),
        _ => match (opcode & 0xF000) {
            0xF000 => match (opcode & 0x0060) {
                0x0060 => load_vx_i(),
                0x0050 => load_i_vx(),
                0x0030 => load_b_vx(),
                0x0020 => load_f_vx(),
                0x0010 => match (opcode & 0x000e) {
                    0x000E => add_i_vx(),
                    0x0008 => load_sound_timer_vx(),
                    0x0005 => unimplemented!(),
                    _ => panic!("Unsupported or corrupt opcode"),
                    },
                0x0000 => match (opcode & 0x000A) {
                    0x000A => unimplemented!(),
                    0x0007 => unimplemented!(),
                    0x0001 => unimplemented!(),
                    _ => panic!("Unsupported or corrupt opcode"),
                    },
                _ => panic!("Unsupported or corrupt opcode"),
            },
            0xE000 => match (opcode & 0x00A0) {
                0x00A0 => unimplemented!(),
                0x0090 => unimplemented!(),
                _ => panic!("unsupported or Corrupt opcode"),
            },
            0xD000 => unimplemented!(),
            0xC000 => unimplemented!(),
            0xB000 => unimplemented!(),
            0xA000 => unimplemented!(),
            0x9000 => unimplemented!(),
            0x8000 => match (opcode & 0x000E) {
                0x000E => unimplemented!(),
                0x0007 => unimplemented!(),
                0x0006 => unimplemented!(),
                0x0005 => unimplemented!(),
                0x0004 => unimplemented!(),
                0x0003 => unimplemented!(),
                0x0002 => unimplemented!(),
                0x0001 => unimplemented!(),
                0x0000 => unimplemented!(),
                _ => panic!("Unsupported or corrupt opcode"),
            },
            0x7000 => unimplemented!(),
            0x8000 => unimplemented!(),
            0x6000 => unimplemented!(),
            0x5000 => unimplemented!(),
            0x4000 => unimplemented!(),
            0x3000 => unimplemented!(),
            0x2000 => unimplemented!(),
            0x1000 => unimplemented!(),
            0x0000 => unimplemented!(),
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
