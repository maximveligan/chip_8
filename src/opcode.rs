use std::fmt;

use nybble::Nybble;
use nybble::ThreeNybbles;
use nybble::TwoNybbles;

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
pub enum Opcode {
    NoArg(NoArg),
    OneArg(OneArg),
    TwoArg(TwoArg),
    ThreeArg(ThreeArg),
}

#[derive(Debug, Clone)]
pub enum NoArg {
    ClearScreen, //00E0
    ReturnSubrt, //00EE
}

#[derive(Clone)]
pub enum OneArg {
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

impl fmt::Debug for OneArg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OneArg::SkipIfVx(nyb) => {
                write!(f, "SkipIfVx {:?} Opcode: Ex9E", nyb)
            }
            OneArg::SkipIfNVx(nyb) => {
                write!(f, "SkipIfNVx {:?} Opcode: ExA1", nyb)
            }
            OneArg::SetVxDT(nyb) => write!(f, "SetVxDT {:?} Opcode: Fx07", nyb),
            OneArg::WaitForKey(nyb) => {
                write!(f, "WaitForKey {:?} Opcode: Fx0A", nyb)
            }
            OneArg::SetDT(nyb) => write!(f, "SetDT {:?} Opcode: Fx15", nyb),
            OneArg::SetST(nyb) => write!(f, "SetST {:?} Opcode: Fx18", nyb),
            OneArg::SetI(nyb) => write!(f, "SetI {:?} Opcode: Fx1E", nyb),
            OneArg::SetSpriteI(nyb) => {
                write!(f, "SetSpriteI {:?} Opcode: Fx29", nyb)
            }
            OneArg::StoreDecVx(nyb) => {
                write!(f, "StoreDecVx {:?} Opcode: Fx33", nyb)
            }
            OneArg::StoreV0Vx(nyb) => {
                write!(f, "StoreV0Vx {:?} Opcode: Fx55", nyb)
            }
            OneArg::ReadV0Vx(nyb) => {
                write!(f, "ReadV0Vx {:?} Opcode: Fx65", nyb)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum TwoArg {
    SkipEqVxVy(TwoNybbles),      //5xy0
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
pub enum ThreeArg {
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
pub enum InvalidOpcode {
    DoesntExist(String, u16),
    StackOverflow(String, Opcode),
    StackUnderflow(String, Opcode),
    OutOfBoundsAddress(String, Opcode),
    NoSuchDigitSprite(String, Opcode),
    OutOfScreenBounds(String, Opcode),
    UndefBehavior(String, Opcode),
}

impl Opcode {
    pub fn decode_op(op: u16) -> Result<Opcode, InvalidOpcode> {
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
            op if ((op & SKIP_IF_NOT_VX) == SKIP_IF_NOT_VX) => {
                Ok(Opcode::OneArg(OneArg::SkipIfNVx(Nybble::from(op))))
            }
            op if ((op & SKIP_IF_VX) == SKIP_IF_VX) => {
                Ok(Opcode::OneArg(OneArg::SkipIfVx(Nybble::from(op))))
            }
            op if ((op & DRAW_VX_VY_NIB) == DRAW_VX_VY_NIB) => Ok(
                Opcode::ThreeArg(ThreeArg::DrawVxVyNib(ThreeNybbles::from(op))),
            ),
            op if ((op & VX_EQ_RAND_PLUS_KK) == VX_EQ_RAND_PLUS_KK) => {
                Ok(Opcode::ThreeArg(ThreeArg::VxEqRandANDKK(
                    ThreeNybbles::from(op),
                )))
            }
            op if ((op & PC_EQ_V0_PLUS_NNN) == PC_EQ_V0_PLUS_NNN) => {
                Ok(Opcode::ThreeArg(ThreeArg::PCEqNNNPlusV0(
                    ThreeNybbles::from(op),
                )))
            }
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
            op if ((op & VX_PLUS_EQ_KK) == VX_PLUS_EQ_KK) => {
                Ok(Opcode::ThreeArg(ThreeArg::VxEqVxPlusKK(
                    ThreeNybbles::from(op),
                )))
            }
            op if ((op & VX_EQ_KK) == VX_EQ_KK) => {
                Ok(Opcode::ThreeArg(ThreeArg::SetVxKK(ThreeNybbles::from(op))))
            }
            op if ((op & SKIP_VX_EQ_VY) == SKIP_VX_EQ_VY) => {
                Ok(Opcode::TwoArg(TwoArg::SkipEqVxVy(TwoNybbles::from(op))))
            }
            op if ((op & SKIP_VX_NEQ_KK) == SKIP_VX_NEQ_KK) => Ok(
                Opcode::ThreeArg(ThreeArg::SkipVxNEqKK(ThreeNybbles::from(op))),
            ),
            op if ((op & SKIP_VX_EQ_KK) == SKIP_VX_EQ_KK) => Ok(
                Opcode::ThreeArg(ThreeArg::SkipVxEqKK(ThreeNybbles::from(op))),
            ),
            op if ((op & CALL_SUB_AT_ADDR) == CALL_SUB_AT_ADDR) => Ok(
                Opcode::ThreeArg(ThreeArg::CallSubAt(ThreeNybbles::from(op))),
            ),
            op if ((op & JUMP_TO_ADDR) == JUMP_TO_ADDR) => Ok(
                Opcode::ThreeArg(ThreeArg::JumpToAddr(ThreeNybbles::from(op))),
            ),
            op if ((op & JUMP_TO_CODEROUTE) == JUMP_TO_CODEROUTE) => {
                Ok(Opcode::ThreeArg(ThreeArg::JumpToCodeRout(
                    ThreeNybbles::from(op),
                )))
            }
            _ => Err(InvalidOpcode::DoesntExist(
                "Unsupported op".to_string(),
                op,
            )),
        }
    }
}
