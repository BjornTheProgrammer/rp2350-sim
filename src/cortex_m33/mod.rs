mod apsr;
pub mod exception;
mod instructions;
pub mod nvic;
pub mod opcodes;
mod operation;
pub mod registers;

use crate::cortex_m33::apsr::Apsr;
use crate::cortex_m33::nvic::Nvic;
use crate::cortex_m33::registers::{CortexM33Registers, Register};
pub use instructions::OpCode;

use self::exception::Exceptions;
use self::operation::get_bits;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Thread,
    Handler,
}

pub struct Shpr {
    shpr2: u32,
    shpr3: u32,
}

impl Shpr {
    pub fn new() -> Self {
        Self { shpr2: 0, shpr3: 0 }
    }

    pub fn pri_11(&self) -> u8 {
        get_bits(self.shpr2, 30..=31) as u8
    }

    pub fn pri_15(&self) -> u8 {
        get_bits(self.shpr3, 30..=31) as u8
    }

    pub fn pri_14(&self) -> u8 {
        get_bits(self.shpr3, 22..=23) as u8
    }
}

pub struct CortexM33 {
    pub registers: CortexM33Registers,
    pub apsr: Apsr,
    pub mode: Mode,
    pub ipsr: u8,
    pub exceptions: Exceptions,
    pub shpr: Shpr,
    pub nvic: Nvic,
}

impl CortexM33 {
    pub fn deafult_exceptions() {}

    pub fn new() -> Self {
        Self {
            registers: CortexM33Registers::new(),
            apsr: Apsr::new(),
            mode: Mode::Thread,
            ipsr: 0,
            exceptions: Exceptions::new(),
            shpr: Shpr::new(),
            nvic: Nvic::new(),
        }
    }

    pub fn get_register_from_number(&mut self, i: u16) -> &mut dyn Register {
        match i {
            0 => &mut self.registers.r0,
            1 => &mut self.registers.r1,
            2 => &mut self.registers.r2,
            3 => &mut self.registers.r3,
            4 => &mut self.registers.r4,
            5 => &mut self.registers.r5,
            6 => &mut self.registers.r6,
            7 => &mut self.registers.r7,
            8 => &mut self.registers.r8,
            9 => &mut self.registers.r9,
            10 => &mut self.registers.r10,
            11 => &mut self.registers.r11,
            12 => &mut self.registers.r12,
            13 => &mut self.registers.sp,
            14 => &mut self.registers.lr,
            15 => &mut self.registers.pc,
            _ => {
                unreachable!()
            }
        }
    }
}
