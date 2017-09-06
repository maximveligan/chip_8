struct Interpreter {
    registers: Registers,
}

struct Registers {
    programCounter: ProgramCounter,
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
            programCounter: ProgramCounter(chip8Adrr),
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
    wholeBank: [u8; 0xFFF],
}

impl Ram {
    fn new() -> Ram {
        Ram {
           wholeBank:  [0; 0xFFF],
        }
    }
}

struct ProgramCounter(u8);

impl ProgramCounter {
    fn updateCounter(&mut self){
        self.0+=2;
    }
}

fn main() {
    let mut ram: Ram = Ram::new();
    let mut registers: Registers = Registers::new();
    loadRom("dev");
    decodeExecuteOpcode(fetchOpcode(&registers.programCounter, ram));
    registers.programCounter.updateCounter();
}

fn loadRom(path: &str) -> () {
    unimplemented!();
}

fn fetchOpcode(pc: &ProgramCounter, ram: Ram) -> u16 {

//  TODO: implement slice references for u8    
    let leftByte: u8 = ram.wholeBank[pc.0 as usize];
    let rightByte: u8 = ram.wholeBank[(pc.0 + 1) as usize];
    (((leftByte as u16) << 8) | (rightByte as u16))
}

fn decodeExecuteOpcode(opcode: u16) -> () {
//  First check to see if opcode has operands
    match opcode {
        0x00E0 => unimplemented!(),
        0x00EE => unimplemented!(),
        _ => match (opcode & 0xF000) {
            0xF000 => match (opcode & 0x0060) {
                0x0060 => unimplemented!(),
                0x0050 => unimplemented!(),
                0x0030 => unimplemented!(),
                0x0020 => unimplemented!(),
                0x0010 => match (opcode & 0x000e) {
                    0x000E => unimplemented!(),
                    0x0008 => unimplemented!(),
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
