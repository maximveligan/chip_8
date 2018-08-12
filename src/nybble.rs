use num::ToPrimitive;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Nybble([u8; 1]);

impl Nybble {
    pub fn new(arg: [u8; 1]) -> Self {
        if arg[0] & (0b11110000) != 0 {
            panic!("Invalid nybble value: {:X}", arg[0]);
        } else {
            Nybble(arg)
        }
    }
}

impl From<u16> for Nybble {
    fn from(op: u16) -> Nybble {
        Nybble::new([((op >> 8) & 0x0F) as u8])
    }
}

impl ToPrimitive for Nybble {
    fn to_i64(&self) -> Option<i64> {
        panic!("Nybbles should only be accessed as unsigned values");
    }
    fn to_u64(&self) -> Option<u64> {
        Some(self.0[0] as u64)
    }
    fn to_usize(&self) -> Option<usize> {
        Some(self.0[0] as usize)
    }
    fn to_u8(&self) -> Option<u8> {
        Some(self.0[0] as u8)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TwoNybbles([u8; 1]);

impl TwoNybbles {
    pub fn x(&self) -> Nybble {
        Nybble::new([(self.0[0] >> 4)])
    }

    pub fn y(&self) -> Nybble {
        Nybble::new([(self.0[0] & 0x0F)])
    }
}

impl From<u16> for TwoNybbles {
    fn from(op: u16) -> TwoNybbles {
        TwoNybbles([((op >> 4) & 0x0FF) as u8])
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ThreeNybbles([u8; 2]);

impl ThreeNybbles {
    pub fn new(arg: [u8; 2]) -> Self {
        if arg[0] & (0b11110000) != 0 {
            panic!(
                "Invalid nybble value: {:X}. Did your three arguments get parsed in correctly?",
                arg[0]
            );
        } else {
            ThreeNybbles(arg)
        }
    }
    pub fn to_addr(&self) -> u16 {
        (((self.0[0] as u16) << 8) | (self.0[1]) as u16)
    }
    pub fn x(&self) -> Nybble {
        Nybble::new([self.0[0]])
    }
    pub fn y(&self) -> Nybble {
        Nybble::new([self.0[1] >> 4])
    }
    pub fn last_nybble(&self) -> u8 {
        (self.0[1] & 0b00001111)
    }

    pub fn get_byte(&self) -> u8 {
        self.0[1]
    }
}

impl From<u16> for ThreeNybbles {
    fn from(op: u16) -> ThreeNybbles {
        ThreeNybbles::new([((op & 0x0F00) >> 8) as u8, (op & 0x00FF) as u8])
    }
}
