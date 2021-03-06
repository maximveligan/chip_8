use nybble::Nybble;
use opcode::Opcode;
use opcode::InvalidOpcode;
use opcode::NoArg;
use opcode::OneArg;
use opcode::TwoArg;
use opcode::ThreeArg;
use screen::Screen;
use keyboard::Keyboard;
use std::fmt;
use num::ToPrimitive;

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
pub struct Cpu {
    pub regs: Registers,
    pub ram: Ram,
    pub stack: Stack,
    pub screen: Screen,
    pub keyboard: Keyboard,
}

impl Cpu {
    pub fn new(bytes: &[u8]) -> Cpu {
        Cpu {
            regs: Registers::new(),
            ram: Ram::initialize_ram(bytes),
            stack: Stack::new(),
            keyboard: Keyboard::new(),
            screen: Screen::new(),
        }
    }

    fn fetch_opcode(&self) -> u16 {
        let l_byte: u8 = self.ram.0[self.regs.pc.get_addr() as usize];
        let r_byte: u8 = self.ram.0[(self.regs.pc.get_addr() + 1) as usize];
        ((l_byte as u16) << 8) | (r_byte as u16)
    }

    pub fn step(&mut self) -> Result<(), InvalidOpcode> {
        self.execute(Opcode::decode_op(self.fetch_opcode())?)
    }

    fn execute(&mut self, op: Opcode) -> Result<(), InvalidOpcode> {
        match op {
            Opcode::NoArg(NoArg::ClearScreen) => {
                self.screen.buffer.iter_mut().for_each(|inner_array| {
                    inner_array.iter_mut().for_each(|pixel| *pixel = false)
                });
                self.regs.pc.update();
                Ok(())
            }
            Opcode::NoArg(NoArg::ReturnSubrt) => {
                match self.stack.pop(&mut self.regs.sp) {
                    Ok(pc) => {
                        self.regs.pc = pc;
                        self.regs.pc.update();
                        Ok(())
                    }
                    Err(err) => Err(InvalidOpcode::StackUnderflow(err, op)),
                }
            }

            Opcode::OneArg(OneArg::SkipIfVx(arg)) => {
                if self.keyboard.key_buffer[self.regs.v_regs
                    [arg.to_usize().expect("Check usize")]
                    as usize]
                {
                    self.regs.pc.update();
                }
                self.regs.pc.update();
                Ok(())
            }
            Opcode::OneArg(OneArg::SkipIfNVx(arg)) => {
                if !self.keyboard.key_buffer[self.regs.v_regs
                    [arg.to_usize().expect("Check usize")]
                    as usize]
                {
                    self.regs.pc.update();
                }
                self.regs.pc.update();
                Ok(())
            }
            Opcode::OneArg(OneArg::SetVxDT(arg)) => {
                self.regs.v_regs[arg.to_usize().expect("Check usize")] =
                    self.regs.delay;
                self.regs.pc.update();
                Ok(())
            }
            Opcode::OneArg(OneArg::WaitForKey(arg)) => {
                self.keyboard.wait_press = Some(arg.to_u8().expect("Check u8"));
                self.regs.pc.update();
                Ok(())
            }
            Opcode::OneArg(OneArg::SetDT(arg)) => {
                self.regs.delay =
                    self.regs.v_regs[arg.to_usize().expect("Check usize")];
                self.regs.pc.update();
                Ok(())
            }
            Opcode::OneArg(OneArg::SetST(arg)) => {
                self.regs.sound =
                    self.regs.v_regs[arg.to_usize().expect("Check usize")];
                self.regs.pc.update();
                Ok(())
            }
            Opcode::OneArg(OneArg::SetI(arg)) => {
                self.regs.i_reg += (self.regs.v_regs
                    [arg.to_usize().expect("Check usize")])
                    as u16;
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
                Err(err) => Err(InvalidOpcode::NoSuchDigitSprite(err, op)),
            },
            Opcode::OneArg(OneArg::StoreDecVx(arg)) => {
                let tmp =
                    self.regs.v_regs[arg.to_usize().expect("Check usize")];
                self.ram.0[self.regs.i_reg as usize] = tmp / 100;
                self.ram.0[(self.regs.i_reg + 1) as usize] = (tmp % 100) / 10;
                self.ram.0[(self.regs.i_reg + 2) as usize] = tmp % 10;
                self.regs.pc.update();
                Ok(())
            }
            Opcode::OneArg(OneArg::StoreV0Vx(arg)) => {
                for index in 0..=arg.to_u64().expect("Check to_u64") {
                    self.ram.0[(self.regs.i_reg + index as u16) as usize] =
                        self.regs.v_regs[index as usize]
                }
                self.regs.pc.update();
                Ok(())
            }
            Opcode::OneArg(OneArg::ReadV0Vx(arg)) => {
                for index in 0..=arg.to_usize().expect("Check usize") {
                    self.regs.v_regs[index as usize] =
                        self.ram.0[(self.regs.i_reg + index as u16) as usize];
                }
                self.regs.pc.update();
                Ok(())
            }
            Opcode::TwoArg(TwoArg::SkipEqVxVy(arg)) => {
                if self.regs.v_regs[arg.x().to_usize().expect("Check usize")]
                    == self.regs.v_regs
                        [arg.y().to_usize().expect("Check usize")]
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
                    & self.regs.v_regs
                        [arg.x().to_usize().expect("Check usize")]
                    == 0b00000001)
                    as u8;
                self.regs.v_regs[arg.x().to_usize().expect("Check usize")] =
                    self.regs.v_regs[arg.x().to_usize().expect("Check usize")]
                        >> 1;
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
                    & self.regs.v_regs
                        [arg.x().to_usize().expect("Check usize")])
                    >> 7 as u8;
                self.regs.v_regs[arg.x().to_usize().expect("Check usize")] =
                    self.regs.v_regs[arg.x().to_usize().expect("Check usize")]
                        << 1;
                self.regs.pc.update();
                Ok(())
            }
            Opcode::TwoArg(TwoArg::SkipVxNEqVy(arg)) => {
                if self.regs.v_regs[arg.x().to_usize().expect("Check usize")]
                    != self.regs.v_regs
                        [arg.y().to_usize().expect("Check usize")]
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
                    Err(err) => Err(InvalidOpcode::StackOverflow(err, op)),
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
                self.regs.v_regs
                    [arg.x().to_usize().expect("Check usize") as usize] =
                    arg.get_byte();
                self.regs.pc.update();
                Ok(())
            }
            Opcode::ThreeArg(ThreeArg::VxEqVxPlusKK(arg)) => {
                self.regs.v_regs[arg.x().to_usize().expect("Check usize")] =
                    self.regs.v_regs[arg.x().to_usize().expect("Check usize")]
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
                let sum =
                    (self.regs.v_regs[0] as usize) + arg.to_addr() as usize;
                if sum > 0xffe || sum < 0x200 {
                    return Err(InvalidOpcode::OutOfBoundsAddress(
                        "Out of bounds program counter".to_string(),
                        op,
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
            Opcode::ThreeArg(ThreeArg::DrawVxVyNib(arg)) => {
                match self.screen.draw_nybble(
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
                    Err(err) => Err(InvalidOpcode::OutOfScreenBounds(err, op)),
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Registers {
    pc: ProgramCounter,
    pub delay: u8,
    pub sound: u8,
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

#[derive(Clone)]
pub struct Ram(Box<[u8]>);

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
    pub fn initialize_ram(bytes: &[u8]) -> Ram {
        let mut ram = Ram {
            0: Box::new([0; 0xFFF]),
        };
        ram.load_digit_data();
        ram.0[0x200..0x200 + bytes.len()].copy_from_slice(bytes);
        ram
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

    pub fn retrieve_bytes(&self, index: u16, amount: Nybble) -> &[u8] {
        &self.0[index as usize
            ..(index as usize) + amount.to_usize().expect("Can't fail")]
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
pub struct Stack([u16; 16]);

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
